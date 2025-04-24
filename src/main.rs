use bevy::prelude::*;

const GRAVITATIONAL_CONSTANT: f32 = 9.8;


#[derive(Component)]
struct Velocity(Vec2);
#[derive(Component)]
struct SmallerBody;
const LARGE_BODY_MASS: f32 = 100.0;
const LARGE_BODY_POSITION: Vec2 = Vec2::new(0.0, 0.0);
const SMALL_BODY_MASS: f32 = 5.0;
const SMALL_BODY_POSITION: Vec2 = Vec2::new(-200.0, 0.0);


fn calculate_acceleration(position: Vec2, mass: f32) -> Vec2 {
    let norm_r: f32 = (position.x.powf(2.0) + position.y.powf(2.0)).sqrt();
    let mu = mass * GRAVITATIONAL_CONSTANT;
    -position * mu / norm_r.powf(3.0)
}

fn calculate_position(acceleration: Vec2, position: Vec2, velocity: &mut Vec2, dt:f32) -> Vec2 {
    *velocity +=  acceleration * dt; // semi implicit Euler method
    position + *velocity * dt
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "N-Body Simulator".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_body)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2d::default());

    let big_body = Circle:: new(LARGE_BODY_MASS / 10.0);
    let small_body = Circle:: new(SMALL_BODY_MASS);
    
    let color = Color::WHITE;

    let distance = SMALL_BODY_POSITION.length();
    let orbital_velocity_magnitude = (GRAVITATIONAL_CONSTANT * LARGE_BODY_MASS / distance).sqrt();
    let initial_velocity = Vec2::new(0.0, orbital_velocity_magnitude);
    
    commands.spawn((
        Mesh2d(meshes.add(big_body)),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(
            LARGE_BODY_POSITION.x,
            LARGE_BODY_POSITION.y,
            0.0,
        ),
    ));
    
    commands.spawn((
        Mesh2d(meshes.add(small_body)),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(
            SMALL_BODY_POSITION.x,
            SMALL_BODY_POSITION.y,
            0.0,
        ),
        SmallerBody,
        Velocity(initial_velocity),
    ));
}

fn move_body(mut query: Query<(&mut Transform, &mut Velocity), With<SmallerBody>>, time: Res<Time>) {
    let dt = time.delta_secs() * 50.0;

    for (mut transform, mut velocity) in &mut query {
        let r = Vec2::new(transform.translation.x, transform.translation.y);
        
        let acceleration = calculate_acceleration(r, LARGE_BODY_MASS);
        let new_position = calculate_position(acceleration, r, &mut velocity.0, dt);

        transform.translation.x = new_position.x;
        transform.translation.y = new_position.y;
    }
}
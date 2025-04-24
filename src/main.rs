use bevy::prelude::*;
use bevy::math::Vec2;
use bevy::prelude::Component;

const SIMULATION_SPEED: f32 = 30.0;
const GRAVITATIONAL_CONSTANT: f32 = 9.7;
const LARGE_BODY_MASS: f32 = 100.0;
const LARGE_BODY_POSITION: Vec2 = Vec2::new(0.0, 0.0);
const SMALL_BODY_MASS: f32 = 20.0;
const SMALL_BODY_POSITION: Vec2 = Vec2::new(-300.0, 0.0);

#[derive(Component)]
pub struct Body {
    pub mass: f32,
    pub position: Vec2,
    pub velocity: Vec2,
}

#[derive(Component)]
struct SmallBody;

impl Body {
    fn update_position(&mut self, dt: f32, mass: f32) -> Vec2 {
        let norm_r: f32 = (self.position.x.powf(2.0) + self.position.y.powf(2.0)).sqrt();
        let mu = mass * GRAVITATIONAL_CONSTANT;
        let acceleration = -self.position * mu / norm_r.powf(3.0);

        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;

        self.position
    }
}

fn calculate_circular_orbit_velocity(body_position: Vec2, attractor_mass: f32) -> Vec2 {
    let distance = body_position.length();
    let orbital_velocity_magnitude = (GRAVITATIONAL_CONSTANT * attractor_mass / distance).sqrt();
    Vec2::new(0.0, orbital_velocity_magnitude)
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2-Body Simulator".into(),
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

    commands.spawn((
        Body {
            mass: LARGE_BODY_MASS,
            position: LARGE_BODY_POSITION,
            velocity: Vec2::new(0.0, 0.0),
        },
        Mesh2d(meshes.add(Circle:: new(LARGE_BODY_MASS))),
         MeshMaterial2d(materials.add(Color::WHITE)),
         Transform::from_xyz(
             LARGE_BODY_POSITION.x,
             LARGE_BODY_POSITION.y,
             0.0,
        )));

    commands.spawn((
        Body {
            mass: SMALL_BODY_MASS,
            position: SMALL_BODY_POSITION,
            velocity: calculate_circular_orbit_velocity(SMALL_BODY_POSITION, LARGE_BODY_MASS),
        },
        
            Mesh2d(meshes.add(Circle:: new(SMALL_BODY_MASS))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(
                SMALL_BODY_POSITION.x,
                SMALL_BODY_POSITION.y,
                0.0,
            ),
        SmallBody
    ));
    
}

fn move_body(mut query: Query<(&mut Transform, &mut Body), With<SmallBody>>, time: Res<Time>) {
    let dt = time.delta_secs() * SIMULATION_SPEED;

    for (mut transform, mut body) in &mut query {
        let new_position = body.update_position(dt, LARGE_BODY_MASS);
        transform.translation.x = new_position.x;
        transform.translation.y = new_position.y;
    }
}

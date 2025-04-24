use bevy::prelude::*;
use bevy::math::Vec2;
use bevy::prelude::Component;

const SIMULATION_SPEED: f32 = 10.0;
const GRAVITATIONAL_CONSTANT: f32 = 1.0;
const BODY_A_MASS: f32 = 20.0;
const BODY_B_MASS: f32 = 10.0;
const BODY_A_POSITION: Vec2 = Vec2::new(50.0, 0.0);
const BODY_B_POSITION: Vec2 = Vec2::new(-50.0, 0.0);

#[derive(Component)]
pub struct Body {
    pub mass: f32,
    pub position: Vec2,
    pub velocity: Vec2,
}

#[derive(Component)]
struct BodyA;
#[derive(Component)]
struct BodyB;

fn calculate_circular_orbit_velocity(position: Vec2, attractor_position: Vec2, attractor_mass: f32) -> Vec2 {
    let direction = position - attractor_position;
    let distance = direction.length();
    let orbital_speed = (GRAVITATIONAL_CONSTANT * attractor_mass / distance).sqrt();

    let tangential = Vec2::new(-direction.y, direction.x).normalize();
    tangential * orbital_speed
}

fn calculate_center_of_mass(body_a: &Body, body_b: &Body) -> Vec2 {
    let total_mass = body_a.mass + body_b.mass;
    let com_x = (body_a.position.x * body_a.mass + body_b.position.x * body_b.mass) / total_mass;
    let com_y = (body_a.position.y * body_a.mass + body_b.position.y * body_b.mass) / total_mass;
    Vec2::new(com_x, com_y)
}

impl Body {
    fn update_position(&mut self, other: &Body, dt: f32) {
        let r = self.position - other.position;
        let norm_r = r.length();
        let mu = GRAVITATIONAL_CONSTANT * other.mass;

        let acceleration = -r * mu / norm_r.powi(3);
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;
        
    }
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

    // let r = BODY_B_POSITION - BODY_A_POSITION;
    // let distance = r.length();
    // let direction = r.normalize();
    // let total_mass = BODY_A_MASS + BODY_B_MASS;
    // let orbital_speed = (GRAVITATIONAL_CONSTANT * total_mass / distance).sqrt();
    // let tangent = Vec2::new(-direction.y, direction.x);
    // 
    // let velocity_a = tangent * orbital_speed * (BODY_B_MASS / total_mass);
    // let velocity_b = -tangent * orbital_speed * (BODY_A_MASS / total_mass);


    let distance = (BODY_B_POSITION - BODY_A_POSITION).length();
    let total_mass = BODY_A_MASS + BODY_B_MASS;
    let orbital_speed = (GRAVITATIONAL_CONSTANT * total_mass / distance).sqrt() * 0.7; // < 1.0 = elliptical

    let direction = (BODY_B_POSITION - BODY_A_POSITION).normalize();
    let tangent = Vec2::new(-direction.y, direction.x);

    let velocity_a = tangent * orbital_speed * (BODY_B_MASS / total_mass);
    let velocity_b = -tangent * orbital_speed * (BODY_A_MASS / total_mass);
    
    commands.spawn((
        Body {
            mass: BODY_A_MASS,
            position: BODY_A_POSITION,
            velocity: velocity_a,
        },
        Mesh2d(meshes.add(Circle:: new(BODY_A_MASS))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(
            BODY_A_POSITION.x,
            BODY_A_POSITION.y,
            0.0, 
         ),
        BodyA,
    ));

    commands.spawn((
        Body {
            mass: BODY_B_MASS,
            position: BODY_B_POSITION,
            velocity: velocity_b,
        },

        Mesh2d(meshes.add(Circle:: new(BODY_B_MASS))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(
            BODY_B_POSITION.x,
            BODY_B_POSITION.y,
            0.0,
            ),
        BodyB
    ));
    
}

fn move_body(mut query: Query<(&mut Transform, &mut Body), Or<(With<BodyA>, With<BodyB>)>>, time: Res<Time>) {
    let dt = time.delta_secs() * SIMULATION_SPEED;

    let mut bodies: Vec<(Mut<Transform>, Mut<Body>)> = query.iter_mut().collect();

    let (mut transform_a, mut body_a) = bodies.remove(0);
    let (mut transform_b, mut body_b) = bodies.remove(0);
    
    body_a.update_position(&body_b, dt);
    body_b.update_position(&body_a, dt);
    
    transform_a.translation.x = body_a.position.x;
    transform_a.translation.y = body_a.position.y;

    transform_b.translation.x = body_b.position.x;
    transform_b.translation.y = body_b.position.y;
}

use bevy::prelude::*;

#[derive(Component)]
struct Body;

const ORBIT_RADIUS: f32 = 100.0;
const ORBIT_SPEED: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_body)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2d::default());

    let big_body = Circle:: new(30.0);
    let small_body = Circle:: new(10.0);
    
    let color = Color::WHITE;

    commands.spawn((
        Mesh2d(meshes.add(big_body)),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(
            0.0,
            0.0,
            0.0,
        ),
    ));
    
    commands.spawn((
        Mesh2d(meshes.add(small_body)),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(
            -50.0,
            0.0,
            0.0,
        ),
        Body,
    ));
}

fn move_body(mut query: Query<&mut Transform, With<Body>>, time: Res<Time>) {
    for mut transform in &mut query {
        let angle = time.elapsed_secs() * ORBIT_SPEED;
        
        transform.translation.x = ORBIT_RADIUS * angle.cos();
        transform.translation.y = ORBIT_RADIUS * angle.sin();
    }
}
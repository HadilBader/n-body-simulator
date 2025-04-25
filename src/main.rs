mod body;
mod simulation;

use bevy::prelude::*;
use bevy::prelude::Component;
use crate::simulation::Simulation;

const DENSITY: f32 = 5.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "N-Body Simulator".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Simulation::three_body_system())
        .add_systems(Startup, setup)
        .add_systems(Update, update_simulation)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, simulation: Res<Simulation>) {
    commands.spawn(Camera2d::default());

    for body in simulation.bodies.iter() {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(5.0))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_translation(Vec3::new(body.position.x, body.position.y, 0.0)),
        ));
    }
    
}

fn update_simulation(mut simulation: ResMut<Simulation>, mut query: Query<(&mut Transform)>) {
    simulation.update();
    for (i, mut transform) in query.iter_mut().enumerate() {
        if let Some(body) = simulation.bodies.get(i) {
            transform.translation.x = body.position.x;
            transform.translation.y = body.position.y;
        }
    }
}

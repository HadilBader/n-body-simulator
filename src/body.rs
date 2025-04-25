use bevy::math::Vec2;
use bevy::prelude::Component;
use crate::simulation::GRAVITATIONAL_CONSTANT;

#[derive(Component)]
pub struct Body {
    pub mass: f32,
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Body {
    pub fn update_position(&mut self, other: &Body, dt: f32) {
        let r = self.position - other.position;
        let norm_r = r.length();
        let mu = GRAVITATIONAL_CONSTANT * other.mass;

        let acceleration = -r * mu / norm_r.powi(3);
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;

    }
}

use std::f32::consts::TAU;
use std::f32;
use bevy::math::Vec2;
use crate::body::Body;
use bevy::prelude::Resource;

pub const GRAVITATIONAL_CONSTANT: f32 = 1.0;
pub const DT: f32 = 0.000_0_1;

#[derive(Resource)]
pub struct Simulation {
    pub bodies: Vec<Body>,
}

impl Simulation {
    pub fn new(bodies: Vec<Body>) -> Self {
        Simulation { bodies }
    }

    pub fn two_body_system() -> Self {
        let body_i = Body {
            mass: 20.0,
            position: Vec2::new(50.0, 0.0),
            velocity:  Vec2::new(-0.0, -0.12780194)
        };
        let body_j = Body {
            mass: 10.0,
            position: Vec2::new(-50.0, 0.0),
            velocity: Vec2::new(0.0, 0.25560388)
        };
        let bodies = vec![body_i, body_j];

        Simulation { bodies }
    }

    pub fn three_body_system() -> Self {
        let mass = 10.0;

        let p1 = Vec2::new(-30.0, -40.0);
        let p2 = Vec2::new(50.0, -20.0);
        let p3 = Vec2::new(-10.0, 60.0);

        let com_x = (p1.x + p2.x + p3.x) / 3.0;
        let com_y = (p1.y + p2.y + p3.y) / 3.0;
        let com = Vec2::new(com_x, com_y);

        let body1 = Body {
            mass,
            position: p1 - com,
            velocity: Vec2::ZERO,
        };
        let body2 = Body {
            mass,
            position: p2 - com,
            velocity: Vec2::ZERO,
        };
        let body3 = Body {
            mass,
            position: p3 - com,
            velocity: Vec2::ZERO,
        };

        let bodies = vec![body1, body2, body3];
        Simulation { bodies }
    }
    
    pub fn update(&mut self) {
        for i in 0..self.bodies.len() {
            for j in i+1..self.bodies.len() {
                let (left, right) = self.bodies.split_at_mut(j);
                let body_i = &mut left[i];
                let body_j = &mut right[0];
                body_i.update_position(body_j, DT);
                body_j.update_position(body_i, DT);
            }
        }
    }
}
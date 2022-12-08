use bevy::prelude::*;

use crate::*;

#[derive(Bundle, Default)]
pub struct ParticleBundle {
    pub pos: Pos,
    pub mass: Mass,
    pub velocity: Velocity,
    pub pre_vel: PreSolveVelocity,
    pub acceleration: Acceleration,
    pub timestep: TimeStep,
    pub collider: CircleCollider,
    pub res: Restitution
}

impl ParticleBundle {
    pub fn new(p: Vec2, r: f32) -> Self {
        Self {
            pos: Pos(p),
            collider: CircleCollider {radius: r},
            ..default()
        }
    }

    pub fn new_with_v(p: Vec2, v: Vec2, r: f32) -> Self {
        Self {
            pos: Pos(p),
            velocity: Velocity(v),
            collider: CircleCollider {radius: r},
            ..default()
        }
    }

    pub fn new_with_v_and_a (p: Vec2, v: Vec2, a: Vec2, r: f32) -> Self {
        Self {
            pos: Pos(p),
            velocity: Velocity(v),
            acceleration: Acceleration(a),
            collider: CircleCollider {radius: r},
            ..default()
        }
    }
}

#[derive(Bundle, Default)]
pub struct StaticBoxBundle {
    pub pos: Pos,
    pub collider: BoxCollider,
    pub res: Restitution,
    pub timestep: TimeStep,
}

impl StaticBoxBundle {
    pub fn new(p: Vec2, s: Vec2) -> Self{
        Self{
            pos: Pos(p),
            collider: BoxCollider {size: s},
            ..default()
        }
    }
}
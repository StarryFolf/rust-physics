use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Pos(pub Vec2);

#[derive(Component)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Component)]
pub struct TimeStep(pub f32);

impl Default for TimeStep {
    fn default() -> Self {
        Self(0.)
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

#[derive(Component)]
pub struct PreSolveVelocity(pub Vec2);

impl Default for PreSolveVelocity {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

#[derive(Component)]
pub struct Acceleration(pub Vec2);

impl Default for Acceleration {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

#[derive(Component)]
pub struct CircleCollider {
    pub radius: f32,
}

impl Default for CircleCollider {
    fn default() -> Self {
        Self {
            radius: 1.0,
        }
    }
}

#[derive(Component)]
pub struct BoxCollider {
    pub size: Vec2,
}

impl Default for BoxCollider {
    fn default() -> Self {
        Self {
            size: Vec2::ONE,
        }
    }
}

#[derive(Component)]
pub struct Restitution(pub f32);

impl Default for Restitution {
    fn default() -> Self {
        Self(1.)
    }
}
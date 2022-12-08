use bevy::prelude::*;

#[derive(Resource)]
pub struct Gravity(pub f32);

impl Default for Gravity {
    fn default() -> Self {
        Self(-9.81)
    }
}

#[derive(Resource, Default)]
pub struct CollisionPairs(pub Vec<(Entity, Entity, Vec2)>);

#[derive(Resource, Default)]
pub struct CollisionPairsStatics(pub Vec<(Entity, Entity, Vec2)>);
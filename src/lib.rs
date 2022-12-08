mod entity;
mod components;
mod resources;

pub use resources::*;
pub use components::*;
pub use entity::*;
use bevy::{prelude::*, time::FixedTimestep};

pub const DELTA_TIME: f32 = 1./120.;

fn diff_eq_horizontal_position(t: f32, _x: f32, v: f32) -> f32{
    v
}

fn runge_kutta_horizontal_position(t: f32, x: f32, dt: f32, v: f32) -> f32{
    let k1 = diff_eq_horizontal_position(t, x, v);
    let k2 = diff_eq_horizontal_position(t + dt/2., x + dt * k1/2., v);
    let k3 = diff_eq_horizontal_position(t + dt/2., x + dt * k2/2., v);
    let k4 = diff_eq_horizontal_position(t + dt, x + dt * k3, v);

    dt/6.*(k1 + 2.*k2 + 2.*k3 + k4)
}

fn diff_eq_vertical_position(t: f32, _y: f32, v: f32) -> f32{
    v
}

fn runge_kutta_vertical_position(t: f32, y: f32, dt: f32, v: f32) -> f32{
    let k1 = diff_eq_vertical_position(t, y, v);
    let k2 = diff_eq_vertical_position(t + dt/2., y + dt * k1/2., v);
    let k3 = diff_eq_vertical_position(t + dt/2., y + dt * k2/2., v);
    let k4 = diff_eq_vertical_position(t + dt, y + dt * k3, v);

    dt/6.*(k1 + 2.*k2 + 2.*k3 + k4)
}
fn diff_eq_horizontal_velocity(t: f32, _vx: f32, a: f32) -> f32{
    a
}

fn runge_kutta_horizontal_velocity(t: f32, vx: f32, dt: f32, a: f32) -> f32{
    let k1 = diff_eq_horizontal_velocity(t, vx, a);
    let k2 = diff_eq_horizontal_velocity(t + dt/2., vx + dt * k1/2., a);
    let k3 = diff_eq_horizontal_velocity(t + dt/2., vx + dt * k2/2., a);
    let k4 = diff_eq_horizontal_velocity(t + dt, vx + dt * k3, a);

    dt/6.*(k1 + 2.*k2 + 2.*k3 + k4)
}

fn diff_eq_vertical_velocity(t: f32, _vy: f32, a: f32) -> f32{
    a
}

fn runge_kutta_vertical_velocity(t: f32, vy: f32, dt: f32, a: f32) -> f32{
    let k1 = diff_eq_vertical_velocity(t, vy, a);
    let k2 = diff_eq_vertical_velocity(t + dt/2., vy + dt * k1/2., a);
    let k3 = diff_eq_vertical_velocity(t + dt/2., vy + dt * k2/2., a);
    let k4 = diff_eq_vertical_velocity(t + dt, vy + dt * k3, a);

    dt/6.*(k1 + 2.*k2 + 2.*k3 + k4)
}

fn collect_colllision_pairs() {}

fn update_pos (mut query: Query<(&mut Pos, &TimeStep, &Velocity, &mut PreSolveVelocity)>) {
    for (mut pos, time, v, mut pre_v) in query.iter_mut() {
        let solve = Vec2::new(
            runge_kutta_horizontal_position(time.0, pos.0.x, DELTA_TIME, v.0.x) * 125.,
            runge_kutta_vertical_position(time.0, pos.0.y, DELTA_TIME, v.0.y) * 125.
        );
        pos.0 += solve;
        pre_v.0 = v.0;
    }
}

fn clear_collide_lists (mut collide_list: ResMut<CollisionPairs>, mut static_collide_list: ResMut<CollisionPairsStatics>) {
    collide_list.0.clear();
    static_collide_list.0.clear();
}

fn solve_pos(mut query: Query<(Entity, &mut Pos, &CircleCollider)>, mut collidelist: ResMut<CollisionPairs>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(entity_a, mut pos_a, collider_a), (entity_b, mut pos_b, collider_b)]) = 
        iter.fetch_next()
    {
        let ab = pos_b.0 - pos_a.0;
        let combined_radius = collider_a.radius + collider_b.radius;
        let ab_sqr_len = ab.length_squared();
        if ab_sqr_len <= combined_radius * combined_radius {
            let ab_length = ab_sqr_len.sqrt();
            let penetration_depth = combined_radius - ab_length;
            let n = ab / ab_length;
            if ab.x >= 0. {      
                pos_a.0 -= n *penetration_depth * 0.5;
                pos_b.0 += n *penetration_depth * 0.5;
            } else {
                pos_a.0 -= n * -penetration_depth * 0.5;
                pos_b.0 += n * -penetration_depth * 0.5;
            }
            collidelist.0.push((entity_a, entity_b, n));
        }
    }
}

fn solve_pos_static_boxes(
    mut dynamics: Query<(Entity, &mut Pos, &CircleCollider), With<Mass>>, 
    statics: Query<(Entity, &Pos, &BoxCollider), Without<Mass>>,
    mut collidelist: ResMut<CollisionPairsStatics>,
) {
    for (entity_a, mut pos_a, collider_a) in dynamics.iter_mut() {
        for (entity_b, pos_b, collider_b) in statics.iter() {
            let box_to_circle = pos_a.0 - pos_b.0;
            let box_to_circle_abs = box_to_circle.abs();
            let half_extents = collider_b.size / 2.;
            let corner_to_center = box_to_circle_abs - half_extents;
            let r = collider_a.radius;
            if corner_to_center.x > r || corner_to_center.y > r {
                continue;
            }

            let s = box_to_circle.signum();

            let (n, penetration_depth) = if corner_to_center.x > 0. && corner_to_center.y > 0. {
                let corner_to_center_sqr = corner_to_center.length_squared();
                if corner_to_center_sqr > r * r {
                    continue;
                }
                let corner_dist = corner_to_center_sqr.sqrt();
                let penetration_depth = r - corner_dist;
                let n = corner_to_center / corner_dist * -s;
                (n, penetration_depth)
            } else if corner_to_center.x > corner_to_center.y {
                // Closer to vertical edge
                (Vec2::X * -s.x, -corner_to_center.x + r)
            } else {
                (Vec2::Y * -s.y, -corner_to_center.y + r)
            };

            pos_a.0 -= n * penetration_depth;
            collidelist.0.push((entity_a, entity_b, n));
        }
    }
}

fn update_vel(mut query: Query<(&mut Velocity, &mut TimeStep, &Acceleration)>, gravity: Res<Gravity>) {
    for (mut v, mut t, a, ) in query.iter_mut() {
        let solve = Vec2::new(
            runge_kutta_horizontal_velocity(t.0, v.0.x, DELTA_TIME, a.0.x),
            runge_kutta_vertical_velocity(t.0, v.0.y, DELTA_TIME, a.0.y + gravity.0)
        );
        v.0 += solve;
        t.0 = t.0 + DELTA_TIME;
    }
}

fn solve_vel(query: Query<(&mut Velocity, &PreSolveVelocity, &Mass, &Restitution)>, collidinglist: Res<CollisionPairs>) {
    for (entity_a, entity_b, n) in collidinglist.0.iter().cloned() {
        let (
            (mut v_a, pre_v_a, m_a, res_a),
            (mut v_b, pre_v_b, m_b, res_b)
        ) = unsafe {
            assert!(entity_a != entity_b);
            (
                query.get_unchecked(entity_a).unwrap(),
                query.get_unchecked(entity_b).unwrap(),
            )
        };
        let pre_solve_relative_vel = pre_v_a.0 - pre_v_b.0;
        let pre_solve_normal_vel = Vec2::dot(pre_solve_relative_vel, n);

        let relative_vel = v_a.0 - v_b.0;
        let normal_vel = Vec2::dot(relative_vel, n);

        let w_a = 1. / m_a.0;
        let w_b = 1. / m_b.0;
        let w_sum = w_a + w_b;

        let restitution = (res_a.0 + res_b.0) / 2.;

        v_a.0 += n * (-normal_vel - restitution * pre_solve_normal_vel) * w_a / w_sum;
        v_b.0 -= n * (-normal_vel - restitution * pre_solve_normal_vel) * w_b / w_sum;
    }
}

fn solve_vel_statics(
    mut dynamics: Query<(&mut Velocity, &mut PreSolveVelocity, &Restitution), With<Mass>>,
    statics: Query<&Restitution, Without<Mass>>,
    contacts: Res<CollisionPairsStatics>,
) {
    for (entity_a, entity_b, n) in contacts.0.iter().cloned() {
        let (mut vel_a, pre_vel_a, restitution_a) = dynamics.get_mut(entity_a).unwrap();
        let restitution_b = statics.get(entity_b).unwrap();
        let pre_solve_normal_vel = Vec2::dot(pre_vel_a.0, n);
        let normal_vel = Vec2::dot(vel_a.0, n);
        let restitution = (restitution_a.0 + restitution_b.0) / 2.;
        vel_a.0 += n * (-normal_vel + (-restitution * pre_solve_normal_vel).min(0.));
    }
}

fn sync_transforms(mut query: Query<(&mut Transform, &Pos)>) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = pos.0.extend(0.);
    }
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum Step {
    CollectCollisionPairs,
    UpdatePositions,
    SolvePositions,
    UpdateVelocities,
    SolveVelocities,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

#[derive(Debug, Default)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Gravity>()
            .init_resource::<CollisionPairs>()
            .init_resource::<CollisionPairsStatics>()
            .add_stage_before(
            CoreStage::Update, 
            FixedUpdateStage, 
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(DELTA_TIME as f64))
                .with_system(
                    collect_colllision_pairs
                        .label(Step::CollectCollisionPairs)
                        .before(Step::UpdatePositions)   
                )
                .with_system(update_pos.label(Step::UpdatePositions))
                .with_system(clear_collide_lists.before(Step::SolvePositions))
                .with_system_set(SystemSet::new()
                    .label(Step::SolvePositions)
                    .after(Step::UpdatePositions)
                    .with_system(solve_pos)
                    .with_system(solve_pos_static_boxes)
                )
                .with_system(
                    update_vel
                        .label(Step::UpdateVelocities)
                        .after(Step::SolvePositions)
                )
                .with_system_set(
                    SystemSet::new()
                        .label(Step::SolveVelocities)
                        .after(Step::UpdateVelocities)
                        .with_system(solve_vel)
                        .with_system(solve_vel_statics)
                )
                .with_system(sync_transforms.after(Step::SolveVelocities))
        );
    }
}

use bevy::{prelude::*, time::FixedTimestep};
use bevy_prototype_lyon::prelude::*;
use rust_physics::*;
use rand::random;

fn startup(mut commands: Commands) {
    let rec_box = shapes::Rectangle {
        origin: RectangleOrigin::Center,
        extents: Vec2::ONE,
        ..default()
    };

    let rec_boc_pos = Vec2::new(0., -300.);
    let rec_boc_size = Vec2::new(500.,250.);

    let rec_box_bundle = GeometryBuilder::build_as(
        &rec_box,
        DrawMode::Fill(FillMode::color(Color::rgb(0.4, 0.4, 0.6))),
        Transform::from_scale(rec_boc_size.extend(1.)).with_translation(rec_boc_pos.extend(0.))
    );

    commands
        .spawn(rec_box_bundle)
        .insert(StaticBoxBundle::new(
            rec_boc_pos,
            rec_boc_size
        ));

    commands
        .spawn(Camera2dBundle::default());
}   

fn spawn_balls(mut commands: Commands) {
    let ball = shapes::Circle {
        radius: 15.,
        ..default()
    };

    let ball_pos = Vec2::new(random::<f32>() * 200., random::<f32>() * 200.) * 0.5 + Vec2::Y * 400.;

    let ball_bundle = GeometryBuilder::build_as(
        &ball,
        DrawMode::Fill(FillMode::color(Color::rgb(0.4, 0.4, 0.6))),
        Transform::from_translation(ball_pos.extend(0.)),
    );

    commands
        .spawn(ball_bundle)
        .insert(ParticleBundle::new_with_v(
            ball_pos,
            Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5),
            ball.radius
        ))
        .insert(Restitution(0.9));
}

fn despawn_balls(mut commands: Commands, query: Query<(Entity, &Pos), With<Mass>>) {
    for (entity, pos) in query.iter() {
        if pos.0.y < -200. {
            commands.entity(entity).despawn();
        }
    }
}

fn main() {
    let window_desc: WindowDescriptor = WindowDescriptor {
        title: "Bevy Physics".to_string(),
        width: 1920.,
        height: 1080.,
        position: WindowPosition::Centered,
        ..default()
    };
    
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.9)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set (WindowPlugin {
            window: window_desc,
            ..default()
        }))
        .add_plugin(ShapePlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(startup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1./20.))
                .with_system(spawn_balls)   
        )
        .add_system(despawn_balls)
        .run();
}

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rust_physics::*;

fn startup(mut commands: Commands) {
    let rec_box = shapes::Rectangle {
        origin: RectangleOrigin::Center,
        extents: Vec2::ONE,
        ..default()
    };

    let rec_boc_pos = Vec2::new(0., 0.);
    let rec_boc_size = Vec2::new(500.,500.);

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

    let circle1 = shapes::Circle {
        radius: 20.,
        ..default()
    };

    let circle1_pos_2d = Vec2::new(-251., 300.);

    let circle_bundle1 = GeometryBuilder::build_as(
        &circle1,
        DrawMode::Fill(FillMode::color(Color::rgb(0.4, 0.4, 0.6))),
        Transform::from_translation(circle1_pos_2d.extend(0.)),
    );

    let circle2 = shapes::Circle {
        radius: 20.,
        ..default()
    };

    let circle2_pos_2d = Vec2::new(251., 300.);

    let circle_bundle2 = GeometryBuilder::build_as(
        &circle2,
        DrawMode::Fill(FillMode::color(Color::rgb(0.4, 0.4, 0.6))),
        Transform::from_translation(circle2_pos_2d.extend(0.)),
    );

    let circle3 = shapes::Circle {
        radius: 20.,
        ..default()
    };

    let circle3_pos_2d = Vec2::new(0., 300.);

    let circle_bundle3 = GeometryBuilder::build_as(
        &circle2,
        DrawMode::Fill(FillMode::color(Color::rgb(0.4, 0.4, 0.6))),
        Transform::from_translation(circle2_pos_2d.extend(0.)),
    );
    
    commands
        .spawn(Camera2dBundle::default());

    commands
        .spawn(circle_bundle1)
        .insert(ParticleBundle::new_with_v(
            circle1_pos_2d,
            Vec2::new(0., 0.),
            (&circle1).radius,
        ));

    commands
        .spawn(circle_bundle2)
        .insert(ParticleBundle::new_with_v(
            circle2_pos_2d,
            Vec2::new(0., 0.),
            (&circle2).radius,
        ));

    commands
    .spawn(circle_bundle3)
    .insert(ParticleBundle::new_with_v(
        circle3_pos_2d,
        Vec2::new(0., 0.),
        (&circle3).radius,
    ))
    .insert(Acceleration(Vec2::new(1., 0.)));
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
        // .insert_resource(Gravity(9.))
        .add_startup_system(startup)
        .run();
}

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rust_physics::*;

fn startup(mut commands: Commands) {
    let circle1 = shapes::Circle {
        radius: 20.,
        ..default()
    };

    let circle_bundle1 = GeometryBuilder::build_as(
        &circle1,
        DrawMode::Outlined { 
            fill_mode: FillMode::color(Color::CYAN), 
            outline_mode: StrokeMode::new(Color::BLACK, 10.), 
        },
        Transform::from_translation(Vec2::new(-30., 20.).extend(0.)),
    );
    
    commands
        .spawn(Camera2dBundle::default());

    commands
        .spawn(circle_bundle1)
        .insert(ParticleBundle::new_with_v(
            Vec2::new(-30., 20.),
            Vec2::new(1., 0.),
            (&circle1).radius
        ));
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
        .run();
}

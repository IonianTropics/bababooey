use std::time::Duration;

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_embedded_assets::EmbeddedAssetPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 360.0,
                height: 240.0,
                title: "Baba Booey".to_owned(),
                cursor_visible: false,
                cursor_grab_mode: CursorGrabMode::Confined,
                ..default()
            },
            close_when_requested: false,
            ..default()
        }).add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin))
        .add_startup_system(setup)
        .add_system(bababooey)
        .run()
}

#[derive(Resource)]
struct BabaTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("JetBrainsMono-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::BLACK,
    };
    let text_alignment = TextAlignment::CENTER;
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Text2dBundle {
            text: Text::from_section("Baba Booey", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
    });
    commands.insert_resource(
        BabaTimer(
            Timer::new(
                Duration::from_secs(1),
                TimerMode::Repeating
            )));
}

fn bababooey(
    time: Res<Time>,
    mut timer: ResMut<BabaTimer>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        let music = asset_server.load("bababooey.ogg");
        audio.play(music);
    }
}

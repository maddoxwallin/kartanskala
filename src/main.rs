use bevy_svg::prelude::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SVG Plugin".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(bevy_svg::prelude::SvgPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg = asset_server.load("Zimbabwe.Svg");
    commands.spawn((Camera2d::default(), Msaa::Sample4));
    commands.spawn((
        Svg2d(svg),
        Origin::Center, // Origin::TopLeft is the default
    ));
}

use bevy::prelude::*;
use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    prelude::*,
};
use bevy_2d_line::LineRenderingPlugin;

use geojson::{Feature, GeoJson, Geometry, Value};
// use std::convert::TryFrom;
use bevy_2d_line::Line;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LineRenderingPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    let geojson_str = r#"
{
  "type": "Feature",
  "properties": { "food": "donuts" },
  "geometry": {
    "type": "MultiPoint",
    "coordinates": [
        [ -101.2836, 34.0956 ],
        [ -1.2836, 3.0956 ],
        [ 11.2836, -34.0956 ]

      ]
  }
  
}
"#;

    let geojson: GeoJson = geojson_str.parse::<GeoJson>().unwrap();
    let feature: Feature = Feature::try_from(geojson).unwrap();

    // read property data
    assert_eq!("donuts", feature.property("food").unwrap());

    // read geometry data
    let geometry: Geometry = feature.geometry.unwrap();
    if let Value::MultiPoint(coords) = geometry.value {
        let coords_1d: Vec<f64> = coords.into_iter().flatten().collect();

        // Convert the 1D vector into a vector of Vec2 instances
        let vec2_coords: Vec<Vec2> = coords_1d
            .chunks(2)
            .map(|chunk| Vec2::new(chunk[0] as f32, chunk[1] as f32))
            .collect();

        let points = vec2_coords.clone();

        println!("{:?}\n", vec2_coords);

        let colors = vec![RED.into(), GREEN.into(), BLUE.into()];

        commands.spawn(Line {
            points,
            colors,
            thickness: 5.0,
        });
    }
}

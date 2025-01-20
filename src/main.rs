// use std::env;
use std::fs;

use bevy::log::tracing_subscriber::fmt::writer::OptionalWriter;
use bevy::prelude::*;
use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    // prelude::*,
};
use bevy_2d_line::LineRenderingPlugin;

use geojson::FeatureCollection;
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

    let geojson: GeoJson = read_file("assets/germany-detailed-boundary_917.geojson")
        .parse::<GeoJson>()
        .unwrap();
    let feature_collection: FeatureCollection = FeatureCollection::try_from(geojson).unwrap();

    // read geometry data
    let features_vec: Vec<Feature> = feature_collection.features;

    for feature in features_vec {
        let geometry: Geometry = feature.geometry.unwrap();
        // if let Value::MultiPoint(coords) = geometry.value {

        let geometry_value = geometry.value;

        println!("{}", &geometry_value);

        let coords = match geometry_value {
            Value::MultiPoint(multipoint) => Some(multipoint),
            _ => None,
        };

        dbg!(&coords);
        // let coords_1d: Vec<f64> = coords.into_iter().flatten().collect();

        // Convert the 1D vector into a vector of Vec2 instances
        // let vec2_coords: Vec<Vec2> = coords_1d
        // .chunks(2)
        // .map(|chunk| Vec2::new(chunk[0] as f32, chunk[1] as f32))
        // .collect();

        // let points = vec2_coords.clone();

        // println!("{:?}\n", vec2_coords);

        // let colors = vec![RED.into(), GREEN.into(), BLUE.into()];

        // commands.spawn(Line {
        //     points,
        //     colors,
        //     thickness: 50000000000000000000000000000.0,
        // });
        // }
    }
}

fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    contents
}

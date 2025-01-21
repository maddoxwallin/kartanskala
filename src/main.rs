// use std::env;
use std::fs;

use bevy_polyline::prelude::*;

use bevy::log::tracing_subscriber::fmt::writer::OptionalWriter;
use bevy::prelude::*;
use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    // prelude::*,
};
use bevy_2d_line::LineRenderingPlugin;

use geojson::*;
// use std::convert::TryFrom;
use bevy_2d_line::Line;

use serde_json::from_str;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PolylinePlugin)
        .add_plugins(LineRenderingPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    let geojson_data = read_file("assets/germany-detailed-boundary_917.geojson");

    let feature_collection: FeatureCollection = from_str(geojson_data.as_str()).unwrap();
    let mut coordinates: Vec<Vec3> = Vec::new();

    for feature in feature_collection.features {
        if let Some(geometry) = feature.geometry {
            match geometry.value {
                Value::MultiPolygon(multipolygon) => {
                    for polygon in multipolygon {
                        for ring in polygon {
                            for coordinate in ring {
                                coordinates.push(Vec3::new(
                                    coordinate[0] as f32,
                                    coordinate[1] as f32,
                                    0.0,
                                ));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    println!("{:?}", &coordinates);

    let points = coordinates.clone();

    commands.spawn(PolylineBundle {
        polyline: PolylineHandle(polylines.add(Polyline { vertices: points })),
        material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
            width: 100.0,
            color: RED.into(),
            perspective: true,
            ..default()
        })),
        ..default()
    });

    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Msaa::Sample4,
        Transform::from_xyz(11.0, 51.0, 10.0),
    ));
}

fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    contents
}

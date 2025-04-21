use bevy::prelude::*;
use bevy::prelude::Resource;

use crate::map::{Map, Tile, MapSize};

const TILE_SIZE: f32 = 10.0;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
           .add_systems(Startup, spawn_map);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_map(mut commands: Commands, map_size: Res<MapSize>) {
    let map_size = *map_size;
    let mut map = Map::new(map_size.width, map_size.height, 42); // <--- à remettre
    map.generate();

    for (y, row) in map.grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let color = match tile {
                Tile::Empty => Color::rgb(0.2, 0.2, 0.2),
                Tile::Obstacle => Color::rgb(0.8, 0.1, 0.1),
                Tile::Energy => Color::rgb(1.0, 1.0, 0.0),
                Tile::Mineral => Color::rgb(0.0, 0.5, 1.0),
                Tile::Science => Color::rgb(0.5, 0.0, 0.8),
                Tile::Station => Color::rgb(0.0, 1.0, 0.0),
                Tile::Bot => Color::rgb(0.0, 1.0, 1.0),
            };

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(10.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    x as f32 * 10.0 - 500.0,
                    -(y as f32 * 10.0) + 300.0,
                    0.0,
                ),
                ..default()
            });
        }
    }

    commands.insert_resource(map);
}


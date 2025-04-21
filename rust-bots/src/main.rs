use bevy::prelude::*;
use bevy::window::WindowPlugin;
use bevy::DefaultPlugins;

mod graphics;
mod map;
mod bot;
mod resources;


use graphics::GraphicsPlugin;
use bot::BotPlugin;
use bevy::prelude::Resource;

use crate::map::{Map, MapSize};


fn main() {
    App::new()
    .insert_resource(MapSize { width: 100, height: 60 })
    .insert_resource(Map::new(100, 60, 42)) // Ajoute la map comme ressource
    .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "EREEA - Map Viewer".to_string(),
                resolution: (1000.0, 800.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(GraphicsPlugin)
        .add_plugins(BotPlugin)
        .run();
}

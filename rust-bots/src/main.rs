use bevy::prelude::*;
use bevy::window::WindowPlugin;
use bevy::DefaultPlugins;

mod graphics;
mod map;
mod bot;
mod resources;
mod ui;
mod station;
mod memory;

use memory::Memory;
use ui::UiPlugin;
use graphics::GraphicsPlugin;
use bot::BotPlugin;
use bevy::prelude::Resource;
use station::Station;

use crate::map::{Map, MapSize};


fn main() {   
    App::new()
    .insert_resource(MapSize { width: 100, height: 60 })
    .insert_resource(Map::new(100, 60, 42))
    .insert_resource(Memory::default()) 
    .insert_resource({
        let mut map = Map::new(100, 60, 42);
        let mut station = Station::new(5, 5, map.clone());
        station.deploy_initial_bots();
        station
    })    .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                title: "EREEA - Map Viewer".to_string(),
                resolution: (1000.0, 800.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(GraphicsPlugin)
        .add_plugins(BotPlugin)
        .add_plugins(UiPlugin)
        .add_systems(Update, run_station) 
        .run();
}
fn run_station(mut station: ResMut<Station>) {
    station.run();
}
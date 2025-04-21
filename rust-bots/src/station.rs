use std::sync::{Arc, Mutex};
use bevy::prelude::Resource;
use rayon::prelude::*;

use crate::bot::{LogicBot, BotType};
use crate::map::{Map, Tile};
use crate::memory::Memory;
use crate::resources::ResourceType;

#[derive(Resource)]
pub struct Station {
    pub x: usize,
    pub y: usize,
    pub bots: Arc<Mutex<Vec<LogicBot>>>,
    pub map: Arc<Mutex<Map>>,
    pub memory: Arc<Mutex<Memory>>,
    pub inventory: Arc<Mutex<usize>>,
}

impl Station {
    pub fn new(x: usize, y: usize, map: Map) -> Self {
        let mut map_copy = map.clone();
        map_copy.grid[y][x] = Tile::Station;

        Self {
            x,
            y,
            bots: Arc::new(Mutex::new(Vec::new())),
            map: Arc::new(Mutex::new(map_copy)),
            memory: Arc::new(Mutex::new(Memory::default())),
            inventory: Arc::new(Mutex::new(0)),
        }
    }

    pub fn deploy_initial_bots(&mut self) {
        let mut bots = self.bots.lock().unwrap();
        bots.push(LogicBot::new(self.x + 1, self.y + 1, BotType::Explorator, self.x, self.y));
        bots.push(LogicBot::new(self.x + 2, self.y, BotType::Collector(ResourceType::Energy), self.x, self.y));
        bots.push(LogicBot::new(self.x, self.y + 2, BotType::Scientist, self.x, self.y));
    
        let mut map = self.map.lock().unwrap();
        for bot in bots.iter() {
            map.grid[bot.y][bot.x] = Tile::Bot;
        }
    }
    

    pub fn run(&mut self) {
        let bots = self.bots.clone();
        bots.lock().unwrap().par_iter_mut().for_each(|bot| {
            let mut map = self.map.lock().unwrap();
            let mut memory = self.memory.lock().unwrap();
            let mut inventory = self.inventory.lock().unwrap();
            bot.act(&mut map, &mut memory, &mut inventory);
        });
    
        let inventory = self.inventory.lock().unwrap();
        let memory = self.memory.lock().unwrap();
        println!("[Station] Total Inventory: {}", *inventory);
        println!("[Station] Memory Size: {} points known.", memory.len());
    }
    
}

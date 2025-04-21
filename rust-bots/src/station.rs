use crate::bot::{Bot, BotType};
use crate::map::{Map, Tile};
use crate::resources::ResourceType;

pub struct Station {
    pub x: usize,
    pub y: usize,
    pub bots: Vec<Bot>,
    pub map: Map,
    pub memory: Vec<(usize, usize, ResourceType)>,
    pub inventory: usize,
}

impl Station {
    pub fn new(x: usize, y: usize, mut map: Map) -> Self {
        map.grid[y][x] = Tile::Station;
        Self {
            x,
            y,
            bots: Vec::new(),
            map,
            memory: Vec::new(),
            inventory: 0,
        }
    }

    pub fn deploy_initial_bots(&mut self) {
        self.bots.push(Bot::new(self.x + 1, self.y + 1, BotType::Explorator, self.x, self.y));
        self.bots.push(Bot::new(self.x + 2, self.y, BotType::Collector(ResourceType::Energy), self.x, self.y));
        self.bots.push(Bot::new(self.x, self.y + 2, BotType::Scientist, self.x, self.y));
        for bot in &self.bots {
            self.map.grid[bot.y][bot.x] = Tile::Bot;
        }
    }

    pub fn run(&mut self) {
        for bot in self.bots.iter_mut() {
            bot.act(&mut self.map, &mut self.memory, &mut self.inventory);
        }
        //self.map.display();
        println!("[Station] Total Inventory: {}", self.inventory);
        println!("[Station] Memory Size: {} points known.", self.memory.len());
    }
}

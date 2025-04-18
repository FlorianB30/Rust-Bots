use crate::bot::Bot; 
use crate::bot::BotType; 
use crate::map::Map; 
use crate::map::Cell;

use std::thread;
use std::time::Duration;

pub struct Station {
    pub bots: Vec<Bot>,
    pub pos_x: usize,
    pub pos_y: usize,
    pub map: Map
}

impl Station {
    pub fn landing(&mut self) {
        self.map.grid[self.pos_y][self.pos_x] = Cell::Station;
        self.start();
    }

    fn start(&mut self) {
       self.deploy_first_bot();
       loop {
        self.map.display();
        thread::sleep(Duration::new(5, 0));
        for bot in &mut self.bots {
           bot.auto();
        }
        self.refresh_bot_location();
       }
    }

    fn deploy_first_bot(&mut self) {
        self.bots.push(
            Bot {
                pos_x: 1, 
                pos_y: 1, 
                type_bot: BotType::Explorator, 
                map_know: self.get_bot_map(),
                bag: 0 
        });
        self.map.grid[1][1] = Cell::Bot;
    }

    fn refresh_bot_location(&mut self) {
        for row in &mut self.map.grid {
            for cell in row.iter_mut() {
                match cell {
                    Cell::Bot => *cell = Cell::Empty,
                    _ => {}
                }
            }
        }

        for bot in &mut self.bots {
            self.map.grid[bot.pos_y][bot.pos_x] = Cell::Bot;
        }
    }

    fn get_bot_map(&self) -> Map {
        let mut map_current = Map {
            width: self.map.width,
            height: self.map.height,
            grid: self.map.grid.clone()
        };

        map_current
    }
}
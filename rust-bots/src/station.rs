use crate::bot::Bot; 
use crate::bot::BotType; 
use crate::map::Map; 
use crate::map::Cell;

pub struct Station {
    pub bots: Vec<Bot>,
    pub pos_x: usize,
    pub pos_y: usize,
    pub map: Map
}

impl Station {
    pub fn landing(&mut self) {
        self.map.grid[self.pos_y][self.pos_x] = Cell::Station;
        self.map.display();
    }

    pub fn start(&mut self) {
       // self.deploy_first_bot();
        self.map.display();
    }

    // fn deploy_first_bot(&mut self) {
    //     self.bots.push(Bot { pos_x: 1, pos_y: 1, type_bot: BotType::Explorator, 
    //         map_know: self.get_bot_map()
    //         , bag: 0 });
    //     self.map.entire_map[1][1] = "O".to_string();
    // }

    // fn get_bot_map(&self) -> Map {
    //     let mut map_current = Map {
    //         x: self.map.x,
    //         y: self.map.y,
    //         entire_map: self.map.entire_map.clone()
    //     };

    //     map_current
    // }
}
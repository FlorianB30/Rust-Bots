use crate::bot::Bot; 
use crate::map::Map; 

pub struct Station {
    pub bots: Vec<Bot>,
    pub pos_x: usize,
    pub pos_y: usize,
    pub map: Map
}

impl Station {
    pub fn landing(&mut self) {
        self.map.entire_map[self.pos_y][self.pos_x] = "S".to_string();
        self.map.display_map();
    }
}
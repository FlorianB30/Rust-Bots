mod bot;

pub struct Station {
    pub bots: Vec<Bot::Bot> = Vec::new(),
    pub pos_x: i32;
    pub pos_y: i32
}
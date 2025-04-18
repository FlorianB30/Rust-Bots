use crate::map::Map; 
use crate::map::Cell;
use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};

pub struct BotInfo {
    pub x: usize,
    pub y: usize,
  //  pub map: Map,
    pub msg: String,
    pub ping: bool
}

pub struct Bot {
    pub pos_x: usize,
    pub pos_y: usize,
    pub type_bot: BotType,
    pub map_know: Map,
    pub bag: i32,
    // pub tx: mpsc::Sender<BotInfo>,
    // pub rx: Arc<Mutex<mpsc::Receiver<BotInfo>>>,
}

pub enum BotType {
    Explorator,
    CollectorEnergy,
    CollectorMineral,
    Scientist
}

impl Bot {
    pub fn auto(&mut self) {
        // 8 cases autour du robot  
        self.action();
    }
    
    pub fn control(&mut self, pos_x: usize, pos_y:usize) {
        // self.communication();
        if self.is_on_map(pos_x, pos_y) {
            self.move_bot(pos_x, pos_y);
        }
    }

    // fn communication(&self) {
    //     let infos = BotInfo {
    //         x: self.pos_x,
    //         y: self.pos_y,
    //         // map: self.map_know,
    //         msg: "Test".to_string(),
    //         ping: true
    //     };
        
    //     let tx_clone = self.tx.clone();
    //     let bot_thread = thread::spawn(move || {
    //         tx_clone.send(infos).unwrap(); 
    //     });

    //     bot_thread.join().unwrap();
    // }

    fn move_bot(&mut self, pos_x: usize, pos_y:usize) {
        self.pos_x = pos_x;
        self.pos_y = pos_y;
    }

    fn isobstacle(&mut self, pos_x: usize, pos_y: usize) -> bool {
        // verifier si il y a des obstacles près du bot
        match self.map_know.get_cell(pos_x, pos_y) {
            Some(Cell::Bot) => true,
            Some(Cell::Station) => true,
            Some(Cell::Obstacle) => true,
            _ => false,
        }
    }

    fn go_home(&mut self) {
        // retour à la station
    }

    fn action(&mut self) {
        match self.type_bot {
            BotType::Explorator => {
                println!("Explore...");
                // découvrir les points près de lui 
                let mut X: usize = self.pos_x;
                let mut Y: usize = self.pos_y;

                for i in 0 .. 7 {
                    if self.is_on_map(X, Y - 1) {
                        if !self.isobstacle(X, Y - 1) {
                            self.move_bot(X, Y - 1);
                            return;
                        }
                    }
                }
            }
            BotType::CollectorEnergy => {
                self.bag += 1;
            }
            BotType::CollectorMineral => {
                self.bag += 1;
            }
            BotType::Scientist => {
                
            }
            _ => println!("Type de bot inconnu."),
        }
    }

    fn is_on_map(&mut self, pos_x: usize, pos_y: usize) -> bool {
        return self.map_know.width >= pos_x && self.map_know.height >= pos_y 
    }
}

pub struct Bot {
    pub pos_x: i32,
    pub pos_y: i32,
    pub type_bot: BotType,
    pub map_know: String
}

pub enum BotType {
    Explorator,
    CollectorEnergy,
    CollectorMineral,
    Scientist
}

impl Bot {
    fn move_bot(&mut self, pos_x: i32, pos_y:i32) {
        self.pos_x = pos_x;
        self.pos_y = pos_y
    }

    fn obstacle(pos_x: i32, pos_y: i32) -> bool {
        // verifier si il y a des obstacles près du bot
        false
    }

    fn go_home(&mut self) {
        // retour à la station
    }

    pub fn action(&mut self) {
        match self.type_bot {
            BotType::Explorator => {
                println!("{}", self.map_know);
            }
            _ => println!("Type de bot inconnu."),
        }
    }
}

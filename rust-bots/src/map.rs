use ansi_term::Colour;

pub struct Map {
    pub x:usize,
    pub y:usize,
    pub entire_map: Vec<Vec<String>>
}

impl Map {
    pub fn generate_map(&mut self) {
        let mut vec_y:Vec<Vec<String>> = Vec::new(); 
        for _i in 0..self.y {  
            let mut vec_x:Vec<String> = Vec::new();
            for _j in 0..self.x {
                vec_x.push(".".to_string());
            }
            vec_y.push(vec_x);
        }
        self.entire_map = vec_y;
    }

    pub fn display_map(&mut self) {
        println!("");
        for y in &self.entire_map {  
            for x in y {
                match x.as_str() {
                    "S" => {
                        print!("{}", Colour::Green.paint("S"));
                    }
                    "O" => {
                        print!("{}", Colour::Green.paint("O"));
                    }
                    _ => {
                        print!("{}", Colour::Fixed(130).paint("."));
                    }
                }
            }
            println!("");
        }
    }
    
}
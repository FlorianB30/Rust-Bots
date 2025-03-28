use ansi_term::Colour;
use noise::{NoiseFn, Perlin};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    ScienceSpot,
    Station,
    Bot
}
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Cell>>,  
}
// pub fn generate_map() {
//     let x = 100; // Largeur
//     let y = 50; // hauteur

//     for _i in 0..y {  
//         for _j in 0..x {
//             print!("{}", Colour::Fixed(130).paint("."));
//         }
//         println!("");
//     }
// }



impl Map {
    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if y < self.height && x < self.width {
            Some(&self.grid[y][x])
        } else {
            None
        }
    }
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        let perlin = Perlin::new(seed);
        let mut grid = vec![vec![Cell::Empty; width]; height];

        for y in 0..height {
            for x in 0..width {
                // let noise_value = perlin.get([x as f64 / 10.0, y as f64 / 10.0]);

                // grid[y][x] = if noise_value > 0.2 {
                //     Cell::Obstacle
                // } else {
                //     Cell::Empty
                // };
                let noise_value = perlin.get([x as f64 / 10.0, y as f64 / 10.0]);

                // Ajustement des seuils pour générer plus de diversité dans la carte
                if noise_value > 0.5 {
                    grid[y][x] = Cell::Obstacle; // Zone d'obstacle plus dense
                } else if noise_value > 0.3 {
                    grid[y][x] = Cell::Energy; // Peut-être de l'énergie
                } else {
                    grid[y][x] = Cell::Empty; // Zone vide
                }

            }
        }

        // Ajout de ressources aléatoires
        let mut rng = rand::thread_rng();
        for _ in 0..5 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            // while grid[y][x] == Cell::Obstacle {
            //     x = rng.gen_range(0..width);
            //     y = rng.gen_range(0..height);
            // }

            grid[y][x] = Cell::Energy;
        }
        for _ in 0..5 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            // while grid[y][x] == Cell::Obstacle {
            //     x = rng.gen_range(0..width);
            //     y = rng.gen_range(0..height);
            // }

            grid[y][x] = Cell::Mineral;
        }
        for _ in 0..3 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            // while grid[y][x] == Cell::Obstacle {
            //     x = rng.gen_range(0..width);
            //     y = rng.gen_range(0..height);
            // }

            grid[y][x] = Cell::ScienceSpot;
        }

        Map { width, height, grid }
    }

    // pub fn display(&self) {
    //     println!("Carte de taille: {}x{}", self.width, self.height); // Affichage des dimensions
    //     for row in &self.grid {
    //         for cell in row {
    //             let symbol = match cell {
    //                 Cell::Empty => ".",
    //                 Cell::Obstacle => "#",
    //                 Cell::Energy => "E",
    //                 Cell::Mineral => "M",
    //                 Cell::ScienceSpot => "S",
    //             };
    //             print!("{}", symbol);
    //         }
    //         println!();
    //     }
    // }
    pub fn display(&self) {
        println!("Carte de taille: {}x{}", self.width, self.height); // Affichage des dimensions
        for row in &self.grid {
            for cell in row {
                let symbol = match cell {
                    Cell::Empty => Colour::Fixed(235).paint("."),  // Gris clair pour vide
                    Cell::Obstacle => Colour::Fixed(160).paint("#"),  // Gris foncé pour obstacle
                    Cell::Energy => Colour::Fixed(46).paint("E"),  // Vert pour l'énergie
                    Cell::Mineral => Colour::Fixed(208).paint("M"),  // Jaune pour le minerai
                    Cell::ScienceSpot => Colour::Fixed(82).paint("S"),  // Bleu pour les spots scientifiques
                    Cell::Station => Colour::Fixed(82).paint("$"),  // Station
                    Cell::Bot => Colour::Fixed(82).paint("¤"),  // robots
                };
                print!("{}", symbol);
            }
            println!();
        }
    }
    
    pub fn collect_resource(&mut self, x: usize, y: usize) -> Option<Cell> {
        match self.grid[y][x] {
            Cell::Energy => {
                self.grid[y][x] = Cell::Empty;  // Effacer la ressource
                Some(Cell::Energy)  // Retourner la ressource collectée
            },
            Cell::Mineral => {
                self.grid[y][x] = Cell::Empty;  // Effacer la ressource
                Some(Cell::Mineral)
            },
            Cell::ScienceSpot => {
                self.grid[y][x] = Cell::Empty;  // Effacer le spot
                Some(Cell::ScienceSpot)
            },
            _ => None,  // Aucune ressource à collecter
        }
    }
    //carte spherique 
    // pub fn spherical_coordinates(x: usize, y: usize, width: usize, height: usize) -> (f64, f64) {
    //     let phi = (x as f64 / width as f64) * 2.0 * std::f64::consts::PI; // Longueur
    //     let theta = (y as f64 / height as f64) * std::f64::consts::PI; // Latitude
    
    //     (phi, theta)
    // }
}


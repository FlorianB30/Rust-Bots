use ansi_term::Colour;
use noise::{NoiseFn, Perlin};
use rand::Rng;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    ScienceSpot,
}

pub fn generate_map() {
    let x = 100; // Largeur
    let y = 50; // hauteur

    for _i in 0..y {  
        for _j in 0..x {
            print!("{}", Colour::Fixed(130).paint("."));
        }
        println!("");
    }
}

pub struct Map {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        let perlin = Perlin::new(seed);
        let mut grid = vec![vec![Cell::Empty; width]; height];

        for y in 0..height {
            for x in 0..width {
                let noise_value = perlin.get([x as f64 / 10.0, y as f64 / 10.0]);

                grid[y][x] = if noise_value > 0.2 {
                    Cell::Obstacle
                } else {
                    Cell::Empty
                };
            }
        }

        // Ajout de ressources aléatoires
        let mut rng = rand::thread_rng();
        for _ in 0..5 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            grid[y][x] = Cell::Energy;
        }
        for _ in 0..5 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            grid[y][x] = Cell::Mineral;
        }
        for _ in 0..3 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            grid[y][x] = Cell::ScienceSpot;
        }

        Map { width, height, grid }
    }

    pub fn display(&self) {
        println!("Carte de taille: {}x{}", self.width, self.height); // Affichage des dimensions
        for row in &self.grid {
            for cell in row {
                let symbol = match cell {
                    Cell::Empty => ".",
                    Cell::Obstacle => "#",
                    Cell::Energy => "E",
                    Cell::Mineral => "M",
                    Cell::ScienceSpot => "S",
                };
                print!("{}", symbol);
            }
            println!();
        }
    }
}


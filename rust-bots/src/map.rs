use noise::{NoiseFn, Perlin};
use bevy::prelude::Resource;


#[derive(Resource, Clone)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub seed: u32,
    pub grid: Vec<Vec<Tile>>,
}

#[derive(Resource, Clone, Copy)]
pub struct MapSize {
    pub width: usize,
    pub height: usize,
}


#[derive(Clone, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    Science,
    Station,
    Bot,
}


impl Map {
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        Self {
            width,
            height,
            seed,
            grid: vec![vec![Tile::Empty; width]; height],
        }
    }

    pub fn generate(&mut self) {
        let noise = Perlin::new(self.seed);
        for y in 0..self.height {
            for x in 0..self.width {
                let val = noise.get([x as f64 / 10.0, y as f64 / 10.0]);
                self.grid[y][x] = if val < -0.4 {
                    Tile::Obstacle
                } else if val > 0.5 {
                    match (x + y) % 3 {
                        0 => Tile::Energy,
                        1 => Tile::Mineral,
                        _ => Tile::Science,
                    }
                } else {
                    Tile::Empty
                };
            }
        }
    }

    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height && !matches!(self.grid[y][x], Tile::Obstacle)
    }
}
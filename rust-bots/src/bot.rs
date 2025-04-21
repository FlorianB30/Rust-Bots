use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::memory::Memory;
use crate::map::{Map, Tile, MapSize};
use crate::resources::ResourceType;

pub const TILE_SIZE: f32 = 10.0;

pub struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bots)
            .add_systems(Update, bot_movement_system);
    }
}

#[derive(Component)]
pub struct Bot;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Debug, Clone)]
pub enum BotType {
    Explorator,
    Collector(ResourceType),
    Scientist,
}

#[derive(Debug, Clone)]
pub struct LogicBot {
    pub x: usize,
    pub y: usize,
    pub bot_type: BotType,
    pub home_x: usize,
    pub home_y: usize,
}

impl LogicBot {
    pub fn new(x: usize, y: usize, bot_type: BotType, home_x: usize, home_y: usize) -> Self {
        Self {
            x,
            y,
            bot_type,
            home_x,
            home_y,
        }
    }

    pub fn act(
        &mut self,
        map: &mut Map,
        memory: &mut Memory,
        inventory: &mut usize,
    ) {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut rng = thread_rng();
        let (dx, dy) = directions.choose(&mut rng).unwrap();

        let new_x = self.x as isize + dx;
        let new_y = self.y as isize + dy;

        if new_x >= 0
            && new_x < map.width as isize
            && new_y >= 0
            && new_y < map.height as isize
        {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            if map.is_valid(new_x, new_y) {
                map.grid[self.y][self.x] = Tile::Empty;
                self.x = new_x;
                self.y = new_y;

                match map.grid[self.y][self.x] {
                    Tile::Energy => memory.add(self.x, self.y, ResourceType::Energy),
                    Tile::Mineral => memory.add(self.x, self.y, ResourceType::Mineral),
                    Tile::Science => memory.add(self.x, self.y, ResourceType::Science),

                    _ => {}
                }

                map.grid[self.y][self.x] = Tile::Bot;
            }
        }

        if self.x == self.home_x && self.y == self.home_y {
            *inventory += memory.len();
            println!(
                "[SYNC] Bot synchronisé avec la station : {} éléments transférés.",
                memory.len()
            );
            memory.clear();
        }
    }
}

fn spawn_bots(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bot_texture: Handle<Image> = asset_server.load("bot.png");

    let positions = vec![
        Vec3::new(-400.0, 200.0, 1.0),
        Vec3::new(-300.0, 200.0, 1.0),
        Vec3::new(-200.0, 200.0, 1.0),
        Vec3::new(-100.0, 200.0, 1.0),
        Vec3::new(0.0, 200.0, 1.0),
    ];

    for pos in positions {
        let direction = random_direction();
        commands.spawn((
            SpriteBundle {
                texture: bot_texture.clone(),
                transform: Transform::from_translation(pos).with_scale(Vec3::splat(0.05)),
                ..default()
            },
            Bot,
            Velocity(direction * 40.0),
        ));
    }
}

fn bot_movement_system(
    time: Res<Time>,
    map: Res<Map>,
    map_size: Res<MapSize>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Bot>>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        let new_pos = transform.translation + velocity.0.extend(0.0) * time.delta_seconds();

        let grid_x = ((new_pos.x + 500.0) / TILE_SIZE).floor() as isize;
        let grid_y = (-(new_pos.y - 300.0) / TILE_SIZE).floor() as isize;

        if grid_x >= 0
            && grid_x < map_size.width as isize
            && grid_y >= 0
            && grid_y < map_size.height as isize
        {
            let gx = grid_x as usize;
            let gy = grid_y as usize;

            if map.is_valid(gx, gy) {
                transform.translation = new_pos;
                continue;
            }
        }

        velocity.0 = random_direction() * 40.0;
    }
}

fn random_direction() -> Vec2 {
    let directions = vec![
        Vec2::new(1.0, 0.0),
        Vec2::new(-1.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(0.0, -1.0),
    ];
    *directions.choose(&mut thread_rng()).unwrap()
}

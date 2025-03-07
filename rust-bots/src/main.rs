mod map;
mod bot;
use std::sync::mpsc;

fn main() {
    // map::generate_map();
    let mut map = map::Map::new(20, 10, 42); 
    
    map.display();

    let positions = [(5, 5), (1, 1), (10, 3), (15, 7)];
    for &(x, y) in &positions {
        println!("\nEssai de collecte à la position ({}, {}):", x, y);
        if let Some(cell) = map.get_cell(x, y) {
            println!("Cellule à la position ({}, {}): {:?}", x, y, cell);
        } else {
            println!("Cellule invalide à la position ({}, {})", x, y);
        }

        if let Some(resource) = map.collect_resource(x, y) {
            println!("Ressource collectée : {:?}", resource);
        } else {
            println!("Aucune ressource à collecter à cette position.");
        }
    }

    let (tx, rx): (mpsc::Sender<bot::BotInfo>, mpsc::Receiver<bot::BotInfo>) = mpsc::channel();

    let mut bot = bot::Bot {
        pos_x: 0,
        pos_y: 0,
        type_bot: bot::BotType::Explorator,
        map_know: map,
        bag: 0,
        tx: tx,
        rx: rx
    };

    // let x = 5;
    // let y = 5;

    // if let Some(cell) = map.get_cell(x, y) {
    //     println!("Cellule à la position ({}, {}): {:?}", x, y, cell);
    // } else {
    //     println!("Cellule invalide à la position ({}, {})", x, y);
    // }

    // //println!("Cellule à la position ({}, {}) : {:?}", x, y, map.grid[y][x]);
    // if let Some(resource) = map.collect_resource(x, y) {
    //     println!("Ressource collectée : {:?}", resource);
    // } else {
    //     println!("Aucune ressource à collecter à cette position.");
    // }

}
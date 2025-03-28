mod map;
mod bot;
mod station;

fn main() {
    // map::generate_map();
    let mut map = map::Map::new(50, 50, 42); 

    let mut station = station::Station{    
        bots: Vec::new(),
        pos_x: 0,
        pos_y: 0,
        map: map
    };

    station.landing();

    // TESTS

    // let positions = [(5, 5), (1, 1), (10, 3), (15, 7)];
    // for &(x, y) in &positions {
    //     println!("\nEssai de collecte à la position ({}, {}):", x, y);
    //     if let Some(cell) = map.get_cell(x, y) {
    //         println!("Cellule à la position ({}, {}): {:?}", x, y, cell);
    //     } else {
    //         println!("Cellule invalide à la position ({}, {})", x, y);
    //     }

    //     if let Some(resource) = map.collect_resource(x, y) {
    //         println!("Ressource collectée : {:?}", resource);
    //     } else {
    //         println!("Aucune ressource à collecter à cette position.");
    //     }
    // }

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

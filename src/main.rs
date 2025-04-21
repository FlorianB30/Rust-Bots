mod map;

fn main() {
    map::generate_map();
    let map = map::Map::new(20, 10, 42); // Créer une carte de 20x10
    map.display(); // Afficher la carte dans le terminal
}
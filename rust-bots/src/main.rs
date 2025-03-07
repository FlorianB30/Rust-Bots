mod map;
mod bot;
mod station;

fn main() { 
    let map = map::Map {
        x: 100,
        y: 50,
        entire_map: Vec::new()
    };

    let mut station = station::Station {
        pos_x: 0,
        pos_y: 0,
        bots: Vec::new(),
        map: map
    };
    
    station.map.generate_map();
    station.map.display_map();
    station.landing();
}

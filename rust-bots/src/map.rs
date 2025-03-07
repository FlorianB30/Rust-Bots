use ansi_term::Colour;

struct Map {
    x:i32,
    y:i32,
    entire_map: String
}

pub fn generate_map() -> String {
    let mut map = Map {
        x: 100,
        y: 50,
        entire_map: "".to_string()
    };

    for _i in 0..map.y {  
        for _j in 0..map.x {
            map.entire_map += ".";
            print!("{}", Colour::Fixed(130).paint("."));
        }
        println!("");
    }

    map.entire_map
}
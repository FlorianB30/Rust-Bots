use ansi_term::Colour;

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
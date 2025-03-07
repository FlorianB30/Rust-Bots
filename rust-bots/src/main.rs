mod map;
mod bot;

fn main() {
    //map::generate_map();
    let mut bot = bot::Bot {
        pos_x: 0,
        pos_y: 0,
        type_bot: bot::BotType::Explorator,
        map_know: "".to_string()
    };

    bot.action();
}

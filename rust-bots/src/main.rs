mod map;
mod bot;

fn main() {
    let map = map::generate_map();
    let mut bot = bot::Bot {
        pos_x: 0,
        pos_y: 0,
        type_bot: bot::BotType::Explorator,
        map_know: map
    };

    bot.action();
}

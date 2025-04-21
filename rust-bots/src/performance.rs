use bevy::prelude::*;
use std::time::Instant;
use crate::station::Station;

pub struct PerformancePlugin;

impl Plugin for PerformancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, log_performance);
    }
}

fn log_performance(station: Res<Station>) {
    let start = Instant::now();

    let bot_count = station.bots.lock().unwrap().len();
    let thread_count = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);

    let duration = start.elapsed();
    println!(
        "[⏱ Performance] Bots actifs: {:<3} | Threads: {} | Frame: {:?}",
        bot_count, thread_count, duration
    );
}

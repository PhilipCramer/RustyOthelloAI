use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rusty_othello_ai::othello::simulate_game;
use std::time::Duration;

pub fn bench_simulate_game(c: &mut Criterion) {
    let mut group = c.benchmark_group("simulate_game");
    group
        .sample_size(1000)
        .measurement_time(Duration::from_secs(10));
    let game_state = rusty_othello_ai::othello::State::new();
    group.bench_function("simulate game 1", |b| {
        b.iter(|| simulate_game(black_box(&mut game_state.clone())))
    });

    group.finish()
}

criterion_group!(game, bench_simulate_game);
criterion_main!(game);

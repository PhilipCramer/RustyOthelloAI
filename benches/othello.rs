use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rusty_othello_ai::{
    mcts::MCTS,
    othello::{simulate_game, State},
};
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
pub fn bench_mcts_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("mcts_search");
    group
        .sample_size(1000)
        .measurement_time(Duration::from_secs(10));
    let mut mcts = MCTS::new("true", 1.0);
    group.bench_function("Monte Carlo Tree Search", |b| {
        b.iter(|| mcts.search(State::new(), 10, |_, _, _| {}))
    });

    group.finish()
}

criterion_group!(game, bench_simulate_game, bench_mcts_search);
criterion_main!(game);

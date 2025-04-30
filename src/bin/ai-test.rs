use rusty_othello_ai::mcts::MCTS;
use rusty_othello_ai::othello::{caculate_win, Color, State};
use std::isize;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut win_balance: isize = 0;
    let a: f32 = args
        .get(1)
        .expect("Missing value for A")
        .parse()
        .expect("Not a valid floatingpoint number");
    let b: f32 = args
        .get(2)
        .expect("Missing value for A")
        .parse()
        .expect("Not a valid floatingpoint number");

    let mut state = State::new();
    let mut mcts = MCTS::new("true", a);
    let mut mcts2 = MCTS::new("false", b);
    let mut ai_iterations = 500;
    loop {
        state = ai_turn(&mut mcts, state.clone(), ai_iterations);
        if state.remaining_moves == 0 {
            break;
        }
        state = ai_turn(&mut mcts2, state.clone(), ai_iterations);
        if state.remaining_moves == 0 {
            break;
        }
        ai_iterations += ai_iterations / 100;
    }
    win_balance += match caculate_win(state) {
        Some(Color::WHITE) => 1,
        Some(Color::BLACK) => -1,
        None => 0,
    };
    println!("{win_balance}")
}

fn ai_turn(mcts: &mut MCTS, state: State, iterations: usize) -> State {
    let dev_null = |_a: usize, _b: usize, _c: &Color| -> () {};
    let action = mcts.search(state.clone(), iterations, dev_null);
    if action.is_ok() {
        state.clone().do_action(Some(action.unwrap().clone()))
    } else {
        state.clone().do_action(None)
    }
}

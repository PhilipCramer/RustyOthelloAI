use rusty_othello_ai::console_game::{ai_turn, ai_turn_2};
use rusty_othello_ai::mcts::MCTS;
use rusty_othello_ai::othello::{caculate_win, print_state, Color, State};
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
    let mut mcts = MCTS::new("false", a);
    let mut mcts2 = MCTS::new("true", b);
    let mut ai_iterations = 20000;
    loop {
        print_state(state);
        //state = ai_turn(&mut mcts, state.clone(), ai_iterations);
        state = ai_turn_2(state, 60, true);
        if state.remaining_moves == 0 {
            break;
        }
        print_state(state);
        //state = ai_turn(&mut mcts2, state.clone(), ai_iterations);
        state = ai_turn_2(state, 1, false);
        if state.remaining_moves == 0 {
            break;
        }
        ai_iterations += ai_iterations / 10;
    }
    win_balance += match caculate_win(&state) {
        Some(Color::WHITE) => {
            println!("\nWhite wins!");
            1
        }
        Some(Color::BLACK) => {
            println!("\nBlack wins!");
            -1
        }
        None => 0,
    };
    println!("{win_balance}")
}

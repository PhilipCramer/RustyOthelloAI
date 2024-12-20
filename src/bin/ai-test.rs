use std::isize;
use rusty_othello_ai::mcts::MCTS;
use rusty_othello_ai::othello::{State,Color};

pub fn main(){
    let args: Vec<String> = std::env::args().collect();
    let mut win_balance: isize = 0;
    let a: f32 = args.get(1).expect("Missing value for A").parse().expect("Not a valid floatingpoint number");
    let b: f32 = args.get(2).expect("Missing value for A").parse().expect("Not a valid floatingpoint number");

    let mut state = State::new();
    let mut mcts = MCTS::new("true", a);
    let mut mcts2 = MCTS::new("false", b);
    let mut ai_iterations = 500;
    loop {
        state = ai_turn(&mut mcts, state.clone(), ai_iterations);
        if state.remaining_moves < 0 {
            break;
        }
        state = ai_turn(&mut mcts2, state.clone(), ai_iterations);
        if state.remaining_moves < 0 {
            break;
        }
        ai_iterations += ai_iterations / 100;
    }
    win_balance += 1;//determine_winner(state);
    println!("{win_balance}")
}
/*
fn determine_winner(state: State) -> isize {
    let p1 = 1;
    let p2 = 0;
    let mut p1_score: isize = 0;
    let mut p2_score: isize = 0;
    for row in state.board {
        for ch in row {
            if ch == p1 {
                p1_score += 1;
            }else if ch == p2 {
                p2_score += 1;
            }
        } 
    }
    match p1_score - p2_score {
        x if x > 0 => 1,
        x if x < 0 => -1,
        _ => 0,
    }
    //println!("Score is\t{} {} : {} {}", p1, p1_score, p2_score, p2);

}
*/

fn ai_turn(mcts: &mut MCTS, state: State, iterations: usize) -> State {
    let dev_null = |_a: usize, _b: usize, _c: &Color | -> (){};
    let action = mcts.search(state.clone(), iterations, dev_null); 
    if action.is_ok() {
        state.clone().do_action(Some(action.unwrap().clone()))
    } 
    else {
        state.clone().do_action(None)
    }
}


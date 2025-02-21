use std::io::Write;
use std::isize;

use crate::mcts::MCTS;
use crate::othello::{caculate_win, print_state, Action, Color, State};

pub fn console_game() {
    let mut win_balance: isize = 0;
    let a = 1.0;
    println!("Game mode: player vs AI\n");
    let mut state = State::new();
    let mut mcts = MCTS::new("true", a);
    _ = std::io::stdout().flush();
    let mut ai_iterations = 5000;
    loop {
        print_state(state);
        state = player_turn(state.clone());
        if state.remaining_moves == 0 {
            break;
        }
        print_state(state);
        state = ai_turn(&mut mcts, state.clone(), ai_iterations);
        ai_iterations += ai_iterations / 100;

        if state.remaining_moves == 0 {
            break;
        }
    }
    //print_state(state);
    win_balance += match caculate_win(state) {
        Some(Color::WHITE) => 1,
        Some(Color::BLACK) => -1,
        None => 0,
    };
    //println!("\nGAME OVER\n");
    println!("\nResult: {win_balance}")
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

fn player_turn(state: State) -> State {
    let mut player_choice: Option<Action>;
    let mut buf = String::new();
    loop {
        print!("Enter coordinates for desired move: ");
        let _ = std::io::stdout().flush();
        let _ = std::io::stdin().read_line(&mut buf);
        let cmd: Vec<&str> = buf.trim().split(",").clone().collect();
        match (cmd.get(0), cmd.get(1)) {
            (Some(cmd_1), Some(cmd_2)) => match (cmd_1.parse::<usize>(), cmd_2.parse::<usize>()) {
                (Ok(y_index), Ok(x_index)) => {
                    player_choice = Some(Action {
                        color: Color::BLACK,
                        x: x_index,
                        y: y_index,
                    });
                    if state
                        .get_actions()
                        .contains(&player_choice.clone().unwrap())
                    {
                        break;
                    } else {
                        println!("Invalid move");
                    }
                }
                _ => println!("Please provide only numbers for indexes"),
            },
            (Some(skip), None) => {
                if skip.to_lowercase() == "skip".to_string() {
                    player_choice = None;
                    break;
                }
            }
            _ => println!("Please provide a move in the form of 1,2 or \"skip\""),
        }
        buf.clear();
    }
    buf.clear();
    state.clone().do_action(player_choice)
}

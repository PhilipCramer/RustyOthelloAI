use std::io::Write;
use std::isize;
use std::process::exit;

use crate::mcts::MCTS;
use crate::othello::{caculate_win, print_state, Action, Color, Position, State};

enum GameCommand {
    SKIP,
    QUIT,
    INVALID,
    MOVE(usize, usize),
}

pub fn console_game() {
    let mut win_balance: isize = 0;
    let a = 1.0;
    println!("Game mode: player vs AI\n");
    let mut state = State::new();
    let mut mcts = MCTS::new("true", a);
    _ = std::io::stdout().flush();
    let mut ai_iterations = 20000;
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
        Some(Color::WHITE) => {
            println!("White wins!");
            1
        }
        Some(Color::BLACK) => {
            println!("Black wins!");
            -1
        }
        None => {
            println!("Draw.");
            0
        }
    };
    //println!("\nGAME OVER\n");
    println!("\nResult: {win_balance}")
}

fn ai_turn(mcts: &mut MCTS, state: State, iterations: usize) -> State {
    let dev_null = |_a: usize, _b: usize, _c: &Color| -> () { /*println!("Progress: {a}/{b}")*/ };
    let action = mcts.search(state.clone(), iterations, dev_null);
    if action.is_ok() {
        println!("{:?}", action.clone().unwrap().position);
        state.clone().do_action(Some(action.unwrap().clone()))
    } else {
        state.clone().do_action(None)
    }
}

fn player_turn(state: State) -> State {
    let mut player_choice;
    loop {
        print!("Enter coordinates for desired move: ");
        let _ = std::io::stdout().flush();
        let cmd = read_command();
        match cmd {
            GameCommand::QUIT => exit(0),
            GameCommand::INVALID => {
                println!("Please provide a valid command 'quit' 'skip' or 'x,y'")
            }
            GameCommand::SKIP => {
                player_choice = None;
                break;
            }
            GameCommand::MOVE(x_index, y_index) => {
                player_choice = Some(Action {
                    color: Color::BLACK,
                    position: Position {
                        x: x_index,
                        y: y_index,
                    },
                });
                if state
                    .get_actions()
                    .contains(&player_choice.clone().unwrap())
                {
                    break;
                } else {
                    println!("Invalid move.");
                    let pos: Vec<(usize, usize)> = state
                        .get_actions()
                        .iter()
                        .map(|a| (a.position.y, a.position.x))
                        .collect();
                    println!("Valid moves: {:?}", pos);
                    print_state(state);
                }
            }
        }
    }
    state.clone().do_action(player_choice)
}

fn read_command() -> GameCommand {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    match buf.to_lowercase().as_str().trim() {
        "quit" => GameCommand::QUIT,
        "skip" => GameCommand::SKIP,
        line => {
            let cmd: Vec<&str> = line.trim().split(",").clone().collect();
            match (cmd.get(0), cmd.get(1)) {
                (Some(cmd_1), Some(cmd_2)) => {
                    match (cmd_1.parse::<usize>(), cmd_2.parse::<usize>()) {
                        (Ok(y_index), Ok(x_index)) => GameCommand::MOVE(x_index, y_index),
                        _ => GameCommand::INVALID,
                    }
                }
                _ => GameCommand::INVALID,
            }
        }
    }
}

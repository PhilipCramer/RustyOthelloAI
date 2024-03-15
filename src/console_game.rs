use std::io::Write;

use crate::mcts::{MCTS, Node};
use crate::othello::{State, Action, print_state};

pub fn console_game(){
    let dev_null = |_a: usize, _b: usize| -> (){};
    let mut state = State::new();
    let mut mcts = MCTS::new(Node::new(state, None, state.get_actions()));
    let mut player_choice: Option<Action>;
    print_state(state);
    let mut buf = String::new();
    while state.remaining_moves > 0 {
        loop {
            print!("Enter coordinates for desired move: ");
            let _ = std::io::stdout().flush();
            let _ = std::io::stdin().read_line(&mut buf);
            let cmd: Vec<&str> = buf.trim().split(",").clone().collect();
            match (cmd.get(0), cmd.get(1)) {
                (Some(cmd_1), Some(cmd_2)) => {
                    match (cmd_1.parse::<usize>(), cmd_2.parse::<usize>()) {
                        (Ok(y_index), Ok(x_index)) => {
                            player_choice = Some(Action {
                                color: 'B',
                                x: x_index,
                                y: y_index,
                            });
                            if state.get_actions().contains(&player_choice.clone().unwrap()) {
                                break;
                            }else {
                                println!("Invalid move");
                            }
                        },
                        _ => println!("Please provide only numbers for indexes"),
                    }
                },
                (Some(skip), None) => {
                    if skip.to_lowercase() == "skip" {
                        player_choice = None;
                        break;
                    }
                },
                _ => println!("Please provide a move in the form of 1,2 or \"skip\""),

            }
            buf.clear();
        }
        buf.clear();
        state = state.do_action(player_choice);
        print_state(state);
        if let Ok(action) = mcts.search(state, 10000, dev_null){
            state = state.do_action(Some(action));
        } else {
            state = state.do_action(None);
        }
        print_state(state);
    }
}

use crate::mcts::{MCTS, Node};
use crate::othello::{State, Action, print_state};

pub fn console_game(){
    let mut state = State::new();
    let mut mcts = MCTS::new(Node::new(state, None, state.get_actions()));
    let mut choice: Result<Action, ()>;
    print_state(state);
    let mut buf = String::new();
    let mut pos: (usize, usize) = (0, 0);
    loop {
        loop {
            print!("Enter coordinates for desired move: ");
            let _ = std::io::stdin().read_line(&mut buf);
            let cmd: Vec<&str> = buf.trim().split(",").clone().collect();
            match (cmd.get(0), cmd.get(1)) {
                (Some(cmd_1), Some(cmd_2)) => {
                    match (cmd_1.parse::<usize>(), cmd_2.parse::<usize>()) {
                        (Ok(x_index), Ok(y_index)) => {
                            pos.0 = x_index;
                            pos.1 = y_index;
                            break;
                        },
                        _ => println!("Please provide only numbers for indexes"),
                    }
                },
                _ => println!("Please provide a move in the form of 1,2 "),

            }
        }

    }
}

use std::io::Write;
use std::isize;

use crate::mcts::MCTS;
use crate::othello::{State, Action};

pub fn console_game(){
    let mut win_balance: isize = 0;
    println!("Playing AI e1.0 vs AI e2.0 \n");
    for i in 1..11 {
        print!("{i}... ");
        let mut state = State::new();
        let mut mcts = MCTS::new("false",1.0);
        let mut mcts2 = MCTS::new("true",2.0);
        _ = std::io::stdout().flush();
        loop {
            state = ai_turn(&mut mcts, state.clone(), 500);
            if state.remaining_moves < 0 {
                break;
            }
            state = ai_turn(&mut mcts2, state.clone(), 500);

            if state.remaining_moves < 0 {
                break;
            }
        }
        //print_state(state);
        win_balance += determine_winner(state);
        //println!("\nGAME OVER\n");
    }
    println!("\nResult: {win_balance}")
}

fn determine_winner(state: State) -> isize {
    let p1 = 'B';
    let p2 = 'W';
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


fn ai_turn(mcts: &mut MCTS, state: State, iterations: usize) -> State {
    let dev_null = |_a: usize, _b: usize| -> (){};
    let action = mcts.search(state.clone(), iterations, dev_null); 
    if action.is_ok() {
        state.clone().do_action(Some(action.unwrap().clone()))
    } 
    else {
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
                if skip.to_lowercase() == "skip".to_string() {
                    player_choice = None;
                    break;
                }
            },
            _ => println!("Please provide a move in the form of 1,2 or \"skip\""),

        }
        buf.clear();
    }
    buf.clear();
    state.clone().do_action(player_choice)

} 

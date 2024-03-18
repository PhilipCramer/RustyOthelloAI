use std::{isize, i16};

use rand::Rng;


const BOARD_SIZE: usize = 8;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct State {
    pub board: [[i8; BOARD_SIZE]; BOARD_SIZE],
    pub next_turn: i8,
    pub remaining_moves: i16,
}
impl State {
    pub fn new() -> Self{
        let mut new = Self {
            board: [
                [-1; BOARD_SIZE]; BOARD_SIZE],
            next_turn: 1,
            remaining_moves: 60,
        };
        new.board[3][3] = 0;
        new.board[3][4] = 1;
        new.board[4][4] = 0;
        new.board[4][3] = 1;
        new

    }
    pub fn get_actions(&self) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();
        let mut tmp_action = Action::new(self.next_turn, 0, 0);
        for (x, row) in self.board.iter().enumerate(){
            for (y, ch) in row.iter().enumerate(){
                tmp_action.x = x;
                tmp_action.y = y;
                if *ch == -1 {
                    for dir in vec![(0,1), (1,0), (1,1), (0,-1), (-1,0), (-1,-1), (1,-1), (-1,1)] {
                        let mut tmp_state = self.clone();
                        if tmp_state.flip_pieces(tmp_action.clone(), dir.0, dir.1){
                            actions.push(tmp_action.clone());
                            break
                        }
                    }   
                }
            }
        }


        return actions;
    }

    pub fn do_action(&mut self, action: Option<Action>) -> State {
        let next_turn = match self.next_turn {
            0 => 1,
            1 => 0,
            _ => -1,
        };

        let mut new_state = State {
            next_turn: next_turn.clone(),
            board: self.board.clone(),
            remaining_moves: (self.remaining_moves.clone() - 1),
        };

        if action.is_some() {
            let act = action.unwrap();
            new_state.board[act.x][act.y] = act.color.clone();
            for dir in vec![(0,1), (1,0), (1,1), (0,-1), (-1,0), (-1,-1), (1,-1), (-1,1)] {
                new_state.flip_pieces(act.clone(), dir.0, dir.1);
            }
        }
        return new_state;
    }

    fn flip_pieces(&mut self, action: Action, x1: isize, y1: isize) -> bool {
        let mut to_flip = Vec::new();
        let mut x_index = (action.x as isize + x1) as usize;
        let mut y_index = (action.y as isize + y1) as usize;
        let own_color = action.color.clone();
        let opponent = match action.color {
            0 => 1,
            _ => 0,
        };
        loop{
            //Bounds Check
            if  x_index > BOARD_SIZE - 1  ||  y_index > BOARD_SIZE - 1 {
                return false;
            }
            match self.board[x_index][y_index] {
                x if x == own_color => break,
                k if k == opponent => {
                    to_flip.push((x_index.clone(), y_index.clone()));
                    x_index = (x_index as isize + x1) as usize;
                    y_index = (y_index as isize + y1) as usize;
                },
                _ => return false,
            }
        }
        if to_flip.len() == 0 {
            return false;
        }
        else {
            for (x,y) in to_flip.iter() {
                self.board[x.clone()][y.clone()] = action.color;
            }
            true
        }
    }
}


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Action {
    pub color: i8,
    pub x: usize,
    pub y: usize,
}

impl Action {
    pub fn new(player: i8, x1: usize, y1: usize) -> Self {
        Self {
            color: player,
            x: x1,
            y: y1,
        }
    }
}


pub fn simulate_game(state: &mut State) -> isize {
    let mut test_state = state.clone();
    let mut test_actions = test_state.get_actions();
    let mut do_act: Option<Action>;
    while test_state.remaining_moves > 0 {
        if test_actions.len() < 1 {
            do_act = None;
        }
        else {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..test_actions.len());
            do_act = test_actions.get(index).cloned();
        }
        test_state = test_state.do_action(do_act);
        test_actions = test_state.get_actions();
    }
    caculate_win(state.next_turn, test_state)
}

fn caculate_win(player: i8, state: State) -> isize {
    let p1 = player;
    let p2 = match p1 {
        1 => 0,
        _ => 1,
    };
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
}

pub fn parse_state(json: serde_json::Value) -> State {
    let mut new_board = [[-1;BOARD_SIZE]; BOARD_SIZE];
    let mut moves_left: i16 = 0;
    let next = match json["turn"] {
        serde_json::Value::Bool(true) => 1,
        _ => 0,

    };
    if let Some(board) = json["board"].as_array() {
        for (x, row) in board.iter().enumerate() {
            if let Some(row) = row.as_array() {
                for (y, cell) in row.iter().enumerate() {
                    match  cell.as_i64() {
                        Some(1) => new_board[x][y] = 1,
                        Some(0) => new_board[x][y] = 0,
                        Some(-1) => {
                            new_board[x][y] = -1;
                            moves_left += 1;
                        },
                        _ => {},
                    }
                }
            }
        }
    }
    State{
        board: new_board,
        next_turn: next,
        remaining_moves: moves_left,
    }
}

pub fn print_state(state: State) {
    println!("   0 1 2 3 4 5 6 7");
    for (i, row) in state.board.iter().enumerate() {
        print!("{i} ");
        for ch in row {
            let c = match ch {
                1 => 'X',
                0 => '0',
                _ => '_',
            };
            print!("|{}", c);
        }
        print!("|\n");
    }
    println!("Next: {:2}", state.next_turn)
}

use rand::Rng;
use std::{isize, u16, usize};

const BOARD_SIZE: usize = 8;
const FIELD_SIZE: usize = 2;
const BLACK_BITMASK: u16 = 0b1010101010101010;
const WHITE_BITMASK: u16 = 0b0101010101010101;
const FLIP_BITMASK: u16 = 0b1100000000000000;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Color {
    BLACK,
    WHITE,
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct State {
    pub board: [u16; BOARD_SIZE],
    pub next_turn: Color,
    pub remaining_moves: u8,
    pub prev_player_skipped: bool,
}
impl State {
    pub fn new() -> Self {
        let mut new = Self {
            board: [0; BOARD_SIZE],
            next_turn: Color::BLACK,
            remaining_moves: 60,
            prev_player_skipped: false,
        };
        let center = (BOARD_SIZE / 2) - 1;
        new.board[3] = 0b0110 << (center * FIELD_SIZE);
        new.board[4] = 0b1001 << (center * FIELD_SIZE);
        new
    }

    pub fn get_actions(&self) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();
        let mut _tmp_action = Action::new(self.next_turn, 0, 0);
        for _row in self.board.iter() {
            // TODO: Fix this
        }
        actions.push(_tmp_action);
        return actions;
    }

    pub fn do_action(&self, action: Option<Action>) -> State {
        let next_turn = match self.next_turn {
            Color::BLACK => Color::WHITE,
            Color::WHITE => Color::BLACK,
        };

        let mut new_state = self.clone();

        if action.is_some() {
            let act = action.unwrap();
            new_state.flip_pieces(act);
        }
        return new_state;
    }

    fn flip_pieces(&mut self, action: Action) -> bool {
        assert!(self.next_turn == action.color);
        let mut result = true;
        result = result && self.flip_row(action.clone());
        result = result && self.flip_column(action.clone());
        result = result && self.flip_diagonals(action);
        return result;
    }
    fn flip_row(&mut self, action: Action) -> bool {
        let row = self.board[action.x];
        let offset_right = action.y * FIELD_SIZE;
        let offset_left = (BOARD_SIZE * FIELD_SIZE) - offset_right;
        let left_of_action = (row >> offset_left) << offset_left;
        let right_of_action = (row << offset_right) >> offset_right;
        if left_of_action != 0 {
            //TODO: Check if valid flip to the left
        }
        if right_of_action != 0 {
            //TODO: Check if valid flip to the left
        }
        //TODO
        return false;
    }
    fn flip_column(&mut self, action: Action) -> bool {
        //TODO
        return false;
    }
    fn flip_diagonals(&mut self, action: Action) -> bool {
        //TODO
        return false;
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Action {
    pub color: Color,
    pub x: usize,
    pub y: usize,
}

impl Action {
    pub fn new(player: Color, x1: usize, y1: usize) -> Self {
        Self {
            color: player,
            x: x1,
            y: y1,
        }
    }
}

pub fn simulate_game(state: &State) -> isize {
    let mut test_state = state.clone();
    let mut test_actions = test_state.get_actions();
    let mut current_action: Option<Action>;
    while test_state.remaining_moves > 0 {
        if test_actions.len() < 1 {
            current_action = None;
        } else {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..test_actions.len());
            current_action = test_actions.get(index).cloned();
        }
        test_state = test_state.do_action(current_action);
        test_actions = test_state.get_actions();
    }
    match caculate_win(test_state) {
        Some(Color::WHITE) => 1,
        Some(Color::BLACK) => -1,
        None => 0,
    }
}

pub fn caculate_win(state: State) -> Option<Color> {
    let mut w_score: isize = 0;
    let mut b_score: isize = 0;
    for row in state.board {
        let (w, b) = count_row(row);
        w_score += w;
        b_score += b;
    }
    match w_score - b_score {
        x if x > 0 => Some(Color::WHITE),
        x if x < 0 => Some(Color::BLACK),
        _ => None,
    }
}
fn count_row(row: u16) -> (isize, isize) {
    let mut w_score = 0;
    let mut b_score = 0;
    let mut b_pieces = (row & BLACK_BITMASK) >> 1;
    let mut w_pieces = row & WHITE_BITMASK;
    for _ in 0..BOARD_SIZE {
        if w_pieces & 0b1 > 0 {
            w_score += 1;
        }
        if b_pieces & 0b1 > 0 {
            b_score += 1;
        }
        b_pieces = b_pieces >> (1 * FIELD_SIZE);
        w_pieces = w_pieces >> (1 * FIELD_SIZE);
    }

    return (w_score, b_score);
}

pub fn parse_state(json: serde_json::Value) -> State {
    let mut new_board = [[-1; BOARD_SIZE]; BOARD_SIZE];
    let mut moves_left: u8 = 0;
    let next = match json["turn"] {
        serde_json::Value::Bool(true) => Color::BLACK,
        _ => Color::WHITE,
    };
    if let Some(board) = json["board"].as_array() {
        for (x, row) in board.iter().enumerate() {
            if let Some(row) = row.as_array() {
                for (y, cell) in row.iter().enumerate() {
                    match cell.as_i64() {
                        Some(1) => new_board[x][y] = 1,
                        Some(0) => new_board[x][y] = 0,
                        Some(-1) => {
                            new_board[x][y] = -1;
                            moves_left += 1;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    State::new()
    /*
    State{
        board: new_board,
        next_turn: next,
        remaining_moves: moves_left,
    }*/
}

pub fn print_state(state: State) {
    println!("   0 1 2 3 4 5 6 7");
    let black_comp = 0b10 << ((BOARD_SIZE - 1) * FIELD_SIZE);
    let white_comp = 0b01 << ((BOARD_SIZE - 1) * FIELD_SIZE);
    for (i, row) in state.board.iter().enumerate() {
        print!("{i} ");
        for f in 0..BOARD_SIZE {
            let c = {
                if row & (black_comp >> (f * FIELD_SIZE)) != 0 {
                    'B'
                } else if row & (white_comp >> (f * FIELD_SIZE)) != 0 {
                    'W'
                } else {
                    '_'
                }
            };
            print!("|{}", c);
        }
        print!("|\n");
    }
    let next = match state.next_turn {
        Color::WHITE => "White",
        Color::BLACK => "Black",
    };
    println!("Next: {}", next)
}

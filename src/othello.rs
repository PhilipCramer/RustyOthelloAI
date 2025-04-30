use rand::Rng;
use std::{fmt, isize, u16, usize};

const BOARD_SIZE: usize = 8;
const FIELD_SIZE: usize = 2;

#[derive(Debug, Clone)]
struct EmptyFieldError;
impl fmt::Display for EmptyFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Empty Fields can't be flipped")
    }
}
#[derive(Debug, Clone)]
struct OccupiedFieldError;
impl fmt::Display for OccupiedFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Occupied Fields can't be Set")
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Color {
    BLACK,
    WHITE,
}
impl Color {
    fn bitmask(&self) -> u16 {
        match *self {
            Color::BLACK => 0b010,
            Color::WHITE => 0b001,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}
impl Direction {
    const VALUES: [Self; 8] = [
        Self::Left,
        Self::Right,
        Self::Up,
        Self::Down,
        Self::UpLeft,
        Self::UpRight,
        Self::DownLeft,
        Self::DownRight,
    ];
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    fn new(x_coordinate: usize, y_coordinate: usize) -> Option<Position> {
        match (x_coordinate, y_coordinate) {
            (x, y) if x >= BOARD_SIZE || y >= BOARD_SIZE => None,
            (_, _) => Some(Self {
                x: x_coordinate,
                y: y_coordinate,
            }),
        }
    }
    fn shift(self, dir: Direction) -> Option<Position> {
        let x = self.x;
        let y = self.y;
        match dir {
            Direction::Up => match y {
                0 => None,
                _ => Position::new(x, y - 1),
            },
            Direction::Down => match y + 1 {
                BOARD_SIZE => None,
                _ => Position::new(x, y + 1),
            },
            Direction::Left => match x {
                0 => None,
                _ => Position::new(x - 1, y),
            },
            Direction::Right => match x + 1 {
                BOARD_SIZE => None,
                _ => Position::new(x + 1, y),
            },
            Direction::UpLeft => match (x, y) {
                (0, _) => None,
                (_, 0) => None,
                (_, _) => Position::new(x - 1, y - 1),
            },
            Direction::UpRight => match (x + 1, y) {
                (BOARD_SIZE, _) => None,
                (_, 0) => None,
                (_, _) => Position::new(x + 1, y - 1),
            },
            Direction::DownLeft => match (x, y + 1) {
                (0, _) => None,
                (_, BOARD_SIZE) => None,
                (_, _) => Position::new(x - 1, y + 1),
            },
            Direction::DownRight => match (x, y) {
                (BOARD_SIZE, _) => None,
                (_, BOARD_SIZE) => None,
                (_, _) => Position::new(x + 1, y + 1),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Row {
    value: u16,
}
impl Row {
    fn new(val: u16) -> Row {
        Self { value: val }
    }
    fn get_pos(&self, pos: usize) -> Option<Color> {
        let mask = 0b011 << (pos * FIELD_SIZE);
        let field = (self.value & mask) >> (pos * FIELD_SIZE);
        match field {
            w if w == Color::WHITE.bitmask() => Some(Color::WHITE),
            b if b == Color::BLACK.bitmask() => Some(Color::BLACK),
            _ => None,
        }
    }
    fn set_pos(&self, color: Color, pos: usize) -> Result<Row, OccupiedFieldError> {
        let color_mask = color.bitmask() << (pos * FIELD_SIZE);
        let check_mask = 0b011 << (pos * FIELD_SIZE);
        match self.value & check_mask {
            0 => Ok(Row {
                value: self.value ^ color_mask,
            }),
            _ => Err(OccupiedFieldError),
        }
    }
    fn flip_pos(&self, pos: usize) -> Result<Row, EmptyFieldError> {
        let flip_mask = 0b011 << (pos * FIELD_SIZE);
        match self.value & flip_mask {
            0 => Err(EmptyFieldError),
            _ => Ok(Row {
                value: self.value ^ flip_mask,
            }),
        }
    }
    fn count_colors(&self) -> (isize, isize) {
        let mut w_score = 0;
        let mut b_score = 0;
        let mut row = self.value.clone();
        for _ in 0..BOARD_SIZE {
            if row & Color::WHITE.bitmask() > 0 {
                w_score += 1;
            }
            if row & Color::BLACK.bitmask() > 0 {
                b_score += 1;
            }
            row = row >> FIELD_SIZE;
        }
        return (w_score, b_score);
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Board {
    rows: [Row; BOARD_SIZE as usize],
}
impl Board {
    fn new() -> Board {
        let mut new_rows = [Row::new(0); BOARD_SIZE as usize];
        let center = (BOARD_SIZE / 2) - 1;
        new_rows[center as usize] = Row::new(0b1001 << (center * FIELD_SIZE));
        new_rows[(center + 1) as usize] = Row::new(0b0110 << (center * FIELD_SIZE));
        Self { rows: new_rows }
    }
    fn blank() -> Board {
        let new_rows = [Row::new(0); BOARD_SIZE as usize];
        Self { rows: new_rows }
    }
    fn flip_pieces(&self, action: Action, position: Position, dir: Direction) -> Option<Board> {
        let mut to_flip = Vec::new();
        let mut current_pos = position;

        // Move in the specified direction, collecting opponent pieces
        while let Some(next_pos) = current_pos.shift(dir) {
            match self.rows[next_pos.y].get_pos(next_pos.x) {
                Some(color) if color != action.color => {
                    // Found an opponent's piece add it to list
                    to_flip.push(next_pos);
                    current_pos = next_pos;
                }
                Some(color) if color == action.color => {
                    // Found own piece flip all the pieces collected
                    if !to_flip.is_empty() {
                        // Create new board with the flipped pieces
                        let mut new_board = self.clone();

                        // Flip all pieces in between
                        for pos in to_flip {
                            new_board.rows[pos.y] = new_board.rows[pos.y]
                                .flip_pos(pos.x)
                                .expect("Should be able to flip occupied positions");
                        }

                        return Some(new_board);
                    }
                    return None;
                }
                _ => {
                    // Empty space or board edge, can't flip in this direction
                    return None;
                }
            }
        }
        None
    }
    fn get_empty_positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for (y, row) in self.into_iter().enumerate() {
            for x in 0..BOARD_SIZE {
                match row.get_pos(x) {
                    None => {
                        positions.push(Position::new(x, y).expect(
                            "Iterating through board shouldn't be able to get out of bounds",
                        ))
                    }
                    Some(_) => (),
                }
            }
        }
        return positions;
    }
    fn would_flip_pieces(&self, action: Action, position: Position, dir: Direction) -> bool {
        match position.shift(dir) {
            Some(pos_1) => match self.rows[pos_1.y].get_pos(pos_1.x) {
                Some(color) if color != action.color => {
                    // Found an opponent's piece in this direction
                    let mut current_pos = pos_1;
                    while let Some(next_pos) = current_pos.shift(dir) {
                        match self.rows[next_pos.y].get_pos(next_pos.x) {
                            Some(color) if color == action.color => {
                                // Found our own piece on the other side
                                return true;
                            }
                            Some(_) => {
                                // Another opponent piece keep checking
                                current_pos = next_pos;
                            }
                            None => {
                                // Empty space can't flip
                                return false;
                            }
                        }
                    }
                    false // Reached edge of board without finding own piece
                }
                _ => false, // Either empty or same color
            },
            None => false, // Can't go in this direction
        }
    }
}
impl IntoIterator for Board {
    type Item = Row;
    type IntoIter = BoardIntoIterator;
    fn into_iter(self) -> Self::IntoIter {
        BoardIntoIterator {
            board: self.clone(),
            index: 0,
        }
    }
}
struct BoardIntoIterator {
    board: Board,
    index: usize,
}
impl Iterator for BoardIntoIterator {
    type Item = Row;
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            x if x < BOARD_SIZE as usize => self.board.rows[x],
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct State {
    board: Board,
    pub next_turn: Color,
    pub remaining_moves: u8,
    pub prev_player_skipped: bool,
}
impl State {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            next_turn: Color::BLACK,
            remaining_moves: 121,
            prev_player_skipped: false,
        }
    }
    pub fn get_actions(&self) -> Vec<Action> {
        let empty_spots = self.board.get_empty_positions();
        let mut actions = Vec::new();
        if empty_spots.len() == 0 {
            return actions;
        }
        for pos in empty_spots {
            let action = Action::new(self.next_turn.clone(), pos);
            if self.is_valid_action(action.clone()) {
                actions.push(action);
            }
        }
        return actions;
    }
    fn is_valid_action(&self, action: Action) -> bool {
        for dir in Direction::VALUES {
            if self
                .board
                .would_flip_pieces(action.clone(), action.position.clone(), dir)
            {
                return true;
            }
        }
        false
    }

    pub fn do_action(&self, action: Option<Action>) -> State {
        let mut new_state = self.clone();
        match action {
            Some(act) => {
                if new_state.flip_directions(act) {
                    new_state.remaining_moves -= 1;
                    new_state.prev_player_skipped = false;
                } else {
                    new_state.prev_player_skipped = true;
                }
            }
            None => {
                new_state.prev_player_skipped = true;
            }
        }
        // If both players had to skip end the game
        if new_state.prev_player_skipped && self.prev_player_skipped {
            new_state.remaining_moves = 0;
        }
        new_state.next_turn = match self.next_turn {
            Color::BLACK => Color::WHITE,
            Color::WHITE => Color::BLACK,
        };
        new_state
    }
    fn flip_directions(&mut self, action: Action) -> bool {
        let mut any_flipped = false;
        let mut new_board = self.board.clone();

        // Set the piece at the action position
        if let Ok(row) = new_board.rows[action.position.y].set_pos(action.color, action.position.x)
        {
            new_board.rows[action.position.y] = row;
        } else {
            return false;
        }
        // Check each direction for pieces to flip
        for dir in Direction::VALUES {
            if let Some(updated_board) = new_board.flip_pieces(action.clone(), action.position, dir)
            {
                new_board = updated_board;
                any_flipped = true;
            }
        }
        if any_flipped {
            self.board = new_board;
            self.remaining_moves -= 1;
        }
        any_flipped
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Action {
    pub color: Color,
    pub position: Position,
}

impl Action {
    pub fn new(player: Color, pos: Position) -> Self {
        Self {
            color: player,
            position: pos,
        }
    }
}

#[inline]
pub fn simulate_game(state: &State) -> isize {
    let mut test_state = state.clone();
    let mut consecutive_skips = 0;

    // Maximum number of moves to prevent infinite loops
    let max_iterations = 100;
    let mut iterations = 0;

    while test_state.remaining_moves > 0 && consecutive_skips < 2 && iterations < max_iterations {
        iterations += 1;

        let test_actions = test_state.get_actions();
        let current_action;

        if test_actions.is_empty() {
            current_action = None;
            consecutive_skips += 1;
        } else {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..test_actions.len());
            current_action = Some(test_actions[index].clone());
            consecutive_skips = 0;
        }

        test_state = test_state.do_action(current_action);

        // If both players had to skip end the game
        if consecutive_skips >= 2 {
            break;
        }
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
    for row in state.board.rows {
        let (w, b) = row.count_colors();
        w_score += w;
        b_score += b;
    }
    match w_score - b_score {
        x if x > 0 => Some(Color::WHITE),
        x if x < 0 => Some(Color::BLACK),
        _ => None,
    }
}

pub fn parse_state(json: serde_json::Value) -> State {
    //todo!("Fix parse_state")
    let mut new_board = Board::blank();
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
                        Some(1) => {
                            new_board.rows[y] = new_board.rows[y].set_pos(Color::WHITE, x).unwrap()
                        }
                        Some(0) => {
                            new_board.rows[y] = new_board.rows[y].set_pos(Color::BLACK, x).unwrap()
                        }
                        Some(-1) => {
                            moves_left += 1;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    State {
        board: new_board,
        next_turn: next,
        remaining_moves: moves_left,
        prev_player_skipped: false,
    }
}

pub fn print_state(state: State) {
    println!("   0 1 2 3 4 5 6 7");
    let black_comp = Color::BLACK.bitmask(); // << ((BOARD_SIZE - 1) * FIELD_SIZE);
    let white_comp = Color::WHITE.bitmask(); // << ((BOARD_SIZE - 1) * FIELD_SIZE);
    for (i, row) in state.board.into_iter().enumerate() {
        print!("{i} ");
        for f in 0..BOARD_SIZE {
            let c = {
                if row.value & (black_comp << (f * FIELD_SIZE)) != 0 {
                    'B'
                } else if row.value & (white_comp << (f * FIELD_SIZE)) != 0 {
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
        Color::BLACK => "Black",
        Color::WHITE => "White",
    };
    println!("Next: {}", next)
}

#[cfg(test)]
mod othello_tests {
    use super::*;

    #[test]
    fn test_board_empty_spaces() {
        let board = Board::new();
        assert_eq!(board.get_empty_positions().len(), 60);
    }
    #[test]
    fn test_row_get_pos() {
        let board = Board::new();
        assert_eq!(board.rows[3].get_pos(3), Some(Color::WHITE));
        assert_eq!(board.rows[3].get_pos(4), Some(Color::BLACK));
        assert_eq!(board.rows[4].get_pos(4), Some(Color::WHITE));
        assert_eq!(board.rows[4].get_pos(3), Some(Color::BLACK));
        assert_eq!(board.rows[1].get_pos(3), None);
        assert_eq!(board.rows[2].get_pos(2), None);
        assert_eq!(board.rows[2].get_pos(4), None);
    }
    #[test]
    fn test_row_set_pos() {
        let board = Board::new();
        assert!(board.rows[3].set_pos(Color::BLACK, 4).is_err());
        assert!(board.rows[3].set_pos(Color::WHITE, 3).is_err());
    }
}

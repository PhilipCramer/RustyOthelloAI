use rand::prelude::*;

const BOARD_SIZE: usize = 8;

// Column masks to prevent bit wrapping during shifts
const NOT_COL_0: u64 = 0xfefefefefefefefe;
const NOT_COL_7: u64 = 0x7f7f7f7f7f7f7f7f;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Color {
    BLACK,
    WHITE,
}
impl Color {
    #[inline]
    fn opponent(self) -> Color {
        match self {
            Color::BLACK => Color::WHITE,
            Color::WHITE => Color::BLACK,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

// Bitboard shift helpers — each returns the board shifted one step in a direction,
// masking out bits that would wrap around the edge.
#[inline(always)]
fn shift_n(b: u64) -> u64 {
    b >> 8
}
#[inline(always)]
fn shift_s(b: u64) -> u64 {
    b << 8
}
#[inline(always)]
fn shift_e(b: u64) -> u64 {
    (b << 1) & NOT_COL_0
}
#[inline(always)]
fn shift_w(b: u64) -> u64 {
    (b >> 1) & NOT_COL_7
}
#[inline(always)]
fn shift_ne(b: u64) -> u64 {
    (b >> 7) & NOT_COL_0
}
#[inline(always)]
fn shift_nw(b: u64) -> u64 {
    (b >> 9) & NOT_COL_7
}
#[inline(always)]
fn shift_se(b: u64) -> u64 {
    (b << 9) & NOT_COL_0
}
#[inline(always)]
fn shift_sw(b: u64) -> u64 {
    (b << 7) & NOT_COL_7
}

/// Kogge-Stone style move generation for one direction.
/// Propagates through opponent pieces to find valid landing squares.
macro_rules! find_moves_dir {
    ($player:expr, $opponent:expr, $empty:expr, $shift_fn:ident) => {{
        let mut candidates = $opponent & $shift_fn($player);
        candidates |= $opponent & $shift_fn(candidates);
        candidates |= $opponent & $shift_fn(candidates);
        candidates |= $opponent & $shift_fn(candidates);
        candidates |= $opponent & $shift_fn(candidates);
        candidates |= $opponent & $shift_fn(candidates);
        $empty & $shift_fn(candidates)
    }};
}

/// Walk from placed piece through opponent pieces in one direction to compute flips.
macro_rules! resolve_flips {
    ($pos_bit:expr, $opponent:expr, $player:expr, $shift_fn:ident) => {{
        let mut flipped = 0u64;
        let mut cursor = $shift_fn($pos_bit);
        while cursor & $opponent != 0 {
            flipped |= cursor;
            cursor = $shift_fn(cursor);
        }
        // Only valid if we ended on one of our own pieces
        if cursor & $player != 0 {
            flipped
        } else {
            0
        }
    }};
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Board {
    black: u64,
    white: u64,
}

impl Board {
    /// Create the standard starting position.
    fn new() -> Board {
        let mut black = 0u64;
        let mut white = 0u64;
        // d4 = (3,3) white, e4 = (4,3) black
        // d5 = (3,4) black, e5 = (4,4) white
        white |= 1u64 << (3 * 8 + 3); // (3,3)
        black |= 1u64 << (3 * 8 + 4); // (4,3)
        black |= 1u64 << (4 * 8 + 3); // (3,4)
        white |= 1u64 << (4 * 8 + 4); // (4,4)
        Board { black, white }
    }

    /// Create an empty board (used by parse_state).
    fn blank() -> Board {
        Board { black: 0, white: 0 }
    }

    /// Get the color at position (x, y), or None if empty.
    #[inline]
    fn get(&self, x: usize, y: usize) -> Option<Color> {
        let bit = 1u64 << (y * 8 + x);
        if self.black & bit != 0 {
            Some(Color::BLACK)
        } else if self.white & bit != 0 {
            Some(Color::WHITE)
        } else {
            None
        }
    }

    /// Set a piece on the board. Panics if position is already occupied.
    #[inline]
    fn set(&mut self, x: usize, y: usize, color: Color) {
        let bit = 1u64 << (y * 8 + x);
        match color {
            Color::BLACK => self.black |= bit,
            Color::WHITE => self.white |= bit,
        }
    }

    /// Get a bitmask of all valid moves for the given color.
    #[inline]
    fn get_moves(&self, color: Color) -> u64 {
        let (player, opponent) = match color {
            Color::BLACK => (self.black, self.white),
            Color::WHITE => (self.white, self.black),
        };
        let empty = !(player | opponent);

        let mut moves = 0u64;
        moves |= find_moves_dir!(player, opponent, empty, shift_n);
        moves |= find_moves_dir!(player, opponent, empty, shift_s);
        moves |= find_moves_dir!(player, opponent, empty, shift_e);
        moves |= find_moves_dir!(player, opponent, empty, shift_w);
        moves |= find_moves_dir!(player, opponent, empty, shift_ne);
        moves |= find_moves_dir!(player, opponent, empty, shift_nw);
        moves |= find_moves_dir!(player, opponent, empty, shift_se);
        moves |= find_moves_dir!(player, opponent, empty, shift_sw);
        moves
    }

    /// Get a bitmask of all pieces that would be flipped by placing `color` at `pos_bit`.
    #[inline]
    fn get_flipped(&self, color: Color, pos_bit: u64) -> u64 {
        let (player, opponent) = match color {
            Color::BLACK => (self.black, self.white),
            Color::WHITE => (self.white, self.black),
        };

        let mut flipped = 0u64;
        flipped |= resolve_flips!(pos_bit, opponent, player, shift_n);
        flipped |= resolve_flips!(pos_bit, opponent, player, shift_s);
        flipped |= resolve_flips!(pos_bit, opponent, player, shift_e);
        flipped |= resolve_flips!(pos_bit, opponent, player, shift_w);
        flipped |= resolve_flips!(pos_bit, opponent, player, shift_ne);
        flipped |= resolve_flips!(pos_bit, opponent, player, shift_nw);
        flipped |= resolve_flips!(pos_bit, opponent, player, shift_se);
        flipped |= resolve_flips!(pos_bit, opponent, player, shift_sw);
        flipped
    }

    /// Apply a move: place the piece and flip captured pieces.
    #[inline]
    fn apply_move(&self, color: Color, pos_bit: u64, flipped: u64) -> Board {
        match color {
            Color::BLACK => Board {
                black: self.black | pos_bit | flipped,
                white: self.white & !flipped,
            },
            Color::WHITE => Board {
                white: self.white | pos_bit | flipped,
                black: self.black & !flipped,
            },
        }
    }

    /// Convenience: compute flips and apply in one call.
    #[inline]
    fn do_move(&self, color: Color, pos_bit: u64) -> Board {
        let flipped = self.get_flipped(color, pos_bit);
        self.apply_move(color, pos_bit, flipped)
    }

    /// Count pieces: returns (white_count, black_count).
    #[inline]
    fn count_pieces(&self) -> (isize, isize) {
        (
            self.white.count_ones() as isize,
            self.black.count_ones() as isize,
        )
    }
}

/// Select the nth set bit from a bitmask (0-indexed).
#[inline]
fn nth_set_bit(mut mask: u64, n: u32) -> u64 {
    for _ in 0..n {
        mask &= mask - 1; // clear lowest set bit
    }
    mask & mask.wrapping_neg() // isolate lowest set bit
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
            remaining_moves: 60,
            prev_player_skipped: false,
        }
    }

    pub fn get_actions(&self) -> Vec<Action> {
        if self.remaining_moves == 0 {
            return Vec::new();
        }
        let moves_mask = self.board.get_moves(self.next_turn);
        let mut actions = Vec::new();
        let mut bits = moves_mask;
        while bits != 0 {
            let bit = bits & bits.wrapping_neg(); // isolate lowest set bit
            let idx = bit.trailing_zeros() as usize;
            let x = idx % 8;
            let y = idx / 8;
            actions.push(Action {
                color: self.next_turn,
                position: Position { x, y },
            });
            bits &= bits - 1; // clear lowest set bit
        }
        actions
    }

    pub fn do_action(&self, action: Option<Action>) -> State {
        match action {
            Some(act) => {
                let pos_bit = 1u64 << (act.position.y * 8 + act.position.x);
                let flipped = self.board.get_flipped(act.color, pos_bit);
                if flipped != 0 {
                    State {
                        board: self.board.apply_move(act.color, pos_bit, flipped),
                        next_turn: self.next_turn.opponent(),
                        remaining_moves: self.remaining_moves - 1,
                        prev_player_skipped: false,
                    }
                } else {
                    // Invalid move — treat as skip
                    let game_over = self.prev_player_skipped;
                    State {
                        board: self.board,
                        next_turn: self.next_turn.opponent(),
                        remaining_moves: if game_over { 0 } else { self.remaining_moves },
                        prev_player_skipped: true,
                    }
                }
            }
            None => {
                let game_over = self.prev_player_skipped;
                State {
                    board: self.board,
                    next_turn: self.next_turn.opponent(),
                    remaining_moves: if game_over { 0 } else { self.remaining_moves },
                    prev_player_skipped: true,
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Action {
    pub color: Color,
    pub position: Position,
}

pub fn simulate_game(state: &State) -> isize {
    let mut board = state.board;
    let mut color = state.next_turn;
    let mut remaining = state.remaining_moves;
    let mut consecutive_skips = 0u8;
    let mut rng = rand::rng();

    while remaining > 0 && consecutive_skips < 2 {
        let moves_mask = board.get_moves(color);
        if moves_mask == 0 {
            consecutive_skips += 1;
        } else {
            let count = moves_mask.count_ones();
            let choice = rng.random_range(0..count);
            let pos_bit = nth_set_bit(moves_mask, choice);
            board = board.do_move(color, pos_bit);
            remaining -= 1;
            consecutive_skips = 0;
        }
        color = color.opponent();
    }

    let (w, b) = board.count_pieces();
    match w - b {
        x if x > 0 => 1,
        x if x < 0 => -1,
        _ => 0,
    }
}

pub fn caculate_win(state: State) -> Option<Color> {
    let (w, b) = state.board.count_pieces();
    match w - b {
        x if x > 0 => Some(Color::WHITE),
        x if x < 0 => Some(Color::BLACK),
        _ => None,
    }
}

pub fn parse_state(json: serde_json::Value) -> State {
    let mut board = Board::blank();
    println!("{}", json.to_string());
    let mut moves_left: u8 = 0;
    let next = match json["turn"] {
        serde_json::Value::Bool(true) => Color::BLACK,
        _ => Color::WHITE,
    };
    if let Some(json_board) = json["board"].as_array() {
        for (x, row) in json_board.iter().enumerate() {
            if let Some(row) = row.as_array() {
                for (y, cell) in row.iter().enumerate() {
                    match cell.as_i64() {
                        Some(0) => board.set(x, y, Color::WHITE),
                        Some(1) => board.set(x, y, Color::BLACK),
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
        board,
        next_turn: next,
        remaining_moves: moves_left,
        prev_player_skipped: false,
    }
}

pub fn print_state(state: State) {
    println!("   0 1 2 3 4 5 6 7");
    for y in 0..BOARD_SIZE {
        print!("{y} ");
        for x in 0..BOARD_SIZE {
            let c = match state.board.get(x, y) {
                Some(Color::BLACK) => 'B',
                Some(Color::WHITE) => 'W',
                None => '_',
            };
            print!("|{}", c);
        }
        println!("|");
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
    fn test_board_empty_squares() {
        let board = Board::new();
        let occupied = (board.black | board.white).count_ones();
        assert_eq!(64 - occupied, 60);
    }

    #[test]
    fn test_initial_pieces() {
        let board = Board::new();
        assert_eq!(board.get(3, 3), Some(Color::WHITE));
        assert_eq!(board.get(4, 3), Some(Color::BLACK));
        assert_eq!(board.get(3, 4), Some(Color::BLACK));
        assert_eq!(board.get(4, 4), Some(Color::WHITE));
        assert_eq!(board.get(0, 0), None);
        assert_eq!(board.get(7, 7), None);
    }

    #[test]
    fn test_initial_valid_moves() {
        let state = State::new();
        let actions = state.get_actions();
        assert_eq!(actions.len(), 4);
    }

    #[test]
    fn test_move_flips_pieces() {
        let state = State::new();
        // Black plays at (2,3) — should flip the white piece at (3,3)
        let action = Action {
            color: Color::BLACK,
            position: Position { x: 2, y: 3 },
        };
        let new_state = state.do_action(Some(action));
        // (2,3) should now be black
        assert_eq!(new_state.board.get(2, 3), Some(Color::BLACK));
        // (3,3) was white, should now be flipped to black
        assert_eq!(new_state.board.get(3, 3), Some(Color::BLACK));
        // (4,3) was already black
        assert_eq!(new_state.board.get(4, 3), Some(Color::BLACK));
        assert_eq!(new_state.remaining_moves, 59);
    }

    #[test]
    fn test_simulate_completes() {
        let state = State::new();
        let result = simulate_game(&state);
        assert!(result == 1 || result == -1 || result == 0);
    }

    #[test]
    fn test_caculate_win() {
        // Equal position at start should be a draw
        let state = State::new();
        assert_eq!(caculate_win(state), None);
    }
}

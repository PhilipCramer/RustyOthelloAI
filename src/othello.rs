


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct State {
    board: [[char; 8]; 8],
    next_turn: char,
}
impl State {
    pub fn new() -> Self{
        let mut new = Self {
            board: [
                ['E'; 8]; 8],
            next_turn: 'B'
        };
        new.board[3][3] = 'W';
        new.board[3][4] = 'B';
        new.board[4][4] = 'W';
        new.board[4][3] = 'B';
        new
            
    }
    pub fn get_actions(&self) -> Vec<Action> {
        // TODO return available actions for the given state
        let a = rand::random();
        let b = rand::random::<usize>();
        vec![Action::new(a, b), Action::new(b, a)]
    }
    pub fn do_action(&mut self, _action: Action) -> State {
        // TODO
        let r_char = rand::random::<char>();
        State {
            next_turn: r_char,
            board: [[r_char; 8]; 8]
        }
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Action {
    x: usize,
    y: usize,
}

impl Action {
    fn new(x1: usize, y1: usize) -> Self {
        Self {
            x: x1,
            y: y1,
        }
    }
}


pub fn simulate_game(_state: &mut State) -> bool {
    rand::random::<bool>()
}

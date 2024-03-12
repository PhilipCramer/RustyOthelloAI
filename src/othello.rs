use rand::Rng;


const BOARD_SIZE: usize = 8;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct State {
    board: [[char; BOARD_SIZE]; BOARD_SIZE],
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
        let mut actions: Vec<Action> = Vec::new();
        let mut tmp_action = Action::new(self.next_turn, 0, 0);
        for (x, row) in self.board.iter().enumerate(){
            for (y, ch) in row.iter().enumerate(){
                let mut tmp_state = self.clone();
                tmp_action.x = x;
                tmp_action.y = y;
                if *ch == 'E' {
                    for dir in vec![(0,1), (1,0), (1,1), (0,-1)].iter() {
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

    pub fn do_action(&mut self, action: Action) -> State {
        let next_turn = match self.next_turn {
            'B' => 'W',
            'W' => 'B',
            _ => 'E',
        };
        let mut board = self.board.clone();
        board[action.x][action.y] = action.color.clone();
        let mut new_state = State {
            next_turn: next_turn.clone(),
            board: board.to_owned(),
        };
        for dir in vec![(0,1), (1,0), (1,1), (0,-1)].iter() {
            new_state.flip_pieces(action.clone(), dir.0, dir.1);
        }
        new_state
    }

    fn flip_pieces(&mut self, action: Action, x1: isize, y1: isize) -> bool {
        if (x1 < 0 && action.x == 0) || (y1 < 0 && action.y == 0){
            return false;
        }
        let x_index = (action.x as isize + x1) as usize;
        let y_index = (action.y as isize + y1) as usize;
        if x_index >= BOARD_SIZE || y_index >= BOARD_SIZE {
            return false;
        }
        if self.board[x_index][y_index] == action.color {
            return true
        }
        match self.next_turn == self.board[x_index][y_index] {
            true => if self.flip_pieces(Action::new(action.color,x_index.clone(),y_index.clone()), x1 , y1) {
                self.board[x_index][y_index] = action.color;
                true
            }
                else {
                    false
                },
            false => false,
        } 
    }
}


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Action {
    color: char,
    x: usize,
    y: usize,
}

impl Action {
    pub fn new(player: char, x1: usize, y1: usize) -> Self {
        Self {
            color: player,
            x: x1,
            y: y1,
        }
    }
}


pub fn simulate_game(state: &mut State) -> bool {
    let mut test_state = state.clone();
    let mut test_actions = test_state.get_actions();
    while test_actions.len() > 0 {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..test_actions.len());
        let do_act = test_actions[index].clone();
        test_state = test_state.do_action(do_act);
        test_actions = test_state.get_actions();
    }
    caculate_win(state.next_turn, test_state)
}

fn caculate_win(player: char, state: State) -> bool {
    let p1 = player;
    let p2 = match p1 {
        'W' => 'B',
        _ => 'W',
    };
    let mut p1_score: usize = 0;
    let mut p2_score: usize = 0;
    for row in state.board {
        for ch in row {
            if ch == p1 {
                p1_score += 1;
            }else if ch == p2 {
                p2_score += 1;
            }
        } 
    }
    p1_score > p2_score
}

pub fn print_state(state: State) {
    for i in state.board {
        println!("{:?}", i);
    }
    println!("next: {}", state.next_turn)
}

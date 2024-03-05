use ureq::{json, Response};
use std::thread;
use std::time::Duration;
use std::collections::{BinaryHeap, HashMap};
use std::cmp:: Ordering;
mod mcts;


const BOARD_SIZE: usize = 8*8;
const SEARCH_DEPTH: usize = 10;

#[derive(Copy, Clone, Eq, PartialEq)]
struct GameState {
    board: [char; BOARD_SIZE],
    cost: usize,
    is_over: bool,

}

impl Ord for GameState {
    fn cmp(&self, other: &GameState) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &GameState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct OthelloMove {
    color: char,
    position: (usize, usize),
}

fn main() {

    //======== This block is just for testing ========================
    let response = get_game_state();
    println!("Received userId: {}", response["userId"]);
    let res = send_move("b6").unwrap();
    print!("Response status: {} {}", res.status(), res.status_text());
    //======== This block is just for testing ========================
}


fn a_star_search(start: [char; BOARD_SIZE], goal) -> Vec<OthelloMove> {
    let mut heap: BinaryHeap<GameState> = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut cost_map = HashMap::new();

    heap.push(GameState {board: start, cost: 0, is_over: false});
    came_from.insert(start, (None, None));
    cost_map.insert(start, 0.0);
    loop {
        let Some(GameState { board, cost, is_over}) = heap.pop();
        if is_over || cost == SEARCH_DEPTH { 
           let mut path: Vec<OthelloMove> = Vec::new();
            let mut current = came_from[&board].0.unwrap();
            while current != start {
                path.push(current.1);
                current = came_from[&current].0.unwrap();
            }
            path.reverse();
            return path.clone(); 
        }
        
        for next_move in get_possible_moves(board).into_iter() {
            let next_state = generate_state(board, next_move);

            heap.push(next_state);
        }
    }
    
    
}

fn get_possible_moves(board: [char;BOARD_SIZE], ) -> Vec<OthelloMove> {
    vec![]
}
fn generate_state(board: [char; BOARD_SIZE] , game_move: OthelloMove) -> GameState {
    GameState { board: board, cost: 42, is_over: true }
}
fn simulate_opponent(state: GameState) -> GameState {
   state 
}



fn get_game_state() -> serde_json::Value {
    let mut delay = Duration::from_millis(10);
     loop {

        match get_json() {
            Ok(resp) => return resp.into_json().unwrap(),
            Err(_e) => {
                thread::sleep(delay);
                delay *= 2;
            },
        }
    }
}

fn get_json() -> Result<Response, ureq::Error> {
    let url = "http://jsonplaceholder.typicode.com/posts/1";
    let resp = ureq::get(url)
        .call();
    Ok(resp?)
}

fn send_move(ai_move: &str) -> Result<Response, ureq::Error> {
    let url = "http://jsonplaceholder.typicode.com/poss";
    let json_body = json!({
        "title": ai_move
    });
    let resp = ureq::post(url)
        .send_json(json_body)?;
    Ok(resp)
}

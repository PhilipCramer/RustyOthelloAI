use ureq::Response;
use std::{thread::sleep, borrow::Borrow};
use std::time::Duration;
mod mcts;
mod othello;
use mcts::{MCTS, Node};
use othello::{State, Action, parse_state};



const SERVER_URL: &str = "http://localhost:8181";

fn main() {
    // Get command line arguments and determine the AI's color
    // The AI color is determined based on the first argument passed to the program
    // If the argument is not recognized, the program will panic
    let args: Vec<String> = std::env::args().collect();
    let ai_color;
    match args.get(1).expect("Please specify color to the AI").to_lowercase() {
        x if x == "false" => ai_color = x,
        x if x == "0" => ai_color = "false".to_string(),
        x if x == "b" => ai_color = "false".to_string(),
        x if x == "black" => ai_color = "false".to_string(),
        x if x == "true" => ai_color = x,
        x if x == "1" => ai_color = "true".to_string(),
        x if x == "w" => ai_color = "true".to_string(),
        x if x == "white" => ai_color = "true".to_string(),
        _ => panic!("Please pass a proper argument to the AI"),

    }
    // Initialize the game state and the Monte Carlo Tree Search (MCTS)
    // The MCTS is initialized with a new node that represents the current game state
    let mut state = State::new();
    let mut mcts = MCTS::new(Node::new(state, None, state.get_actions()));
    let mut choice: Result<Action, ()>;

    // The main game loop
    loop {
        // The AI checks if it's its turn, if so, it gets the current game state and performs a search using MCTS
        match is_my_turn(ai_color.borrow()) {
            Ok(true) =>  {
                state = get_game_state();
                choice = mcts.search(state, 10000);

                // If a valid action is found, it sends the move to the server and updates the game state
                if choice.is_ok() {
                    let _ = send_move(&ai_color, Some(choice.clone().unwrap()));
                    state.do_action(Some(choice.unwrap()));
                }
                // If no valid action is found, it sends a pass move to the server and updates the game state
                else {
                    let _ = send_move(&ai_color, None);
                    state.do_action(None); 
                }
                _ = mcts.search(state, 20000);

            },
            // If it's not the AI's turn, it performs a search using MCTS and waits
            Ok(false) => {
                _ = mcts.search(state, 20000);
                //sleep(Duration::from_secs(1));
            },
            Err(e) => {
                eprintln!("Error checking turn: {}", e);
                sleep(Duration::from_secs(1));
            }
        }
    }
}
// Function to check if it's the AI's turn
// This function makes a GET request to the server and parses the response
fn is_my_turn(ai: &String) -> Result<bool, Box<dyn std::error::Error>> {
    let mut delay = Duration::from_secs(1);
    let opponent = match ai {
        x if x == "true" => "false",
        _ => "true"
    };
    loop {
        let url = format!("{}/turn", SERVER_URL);
        match ureq::get(&url).call() {
            Ok(response) => {
                let body = response.into_string()?;
                match body.trim() {
                    // If the response is "true", it's the AI's turn and the function returns Ok(true)
                    x if x == ai => return Ok(true),
                    // If the response is "false", it's not the AI's turn and the function returns Ok(false)
                    x if x == opponent => return Ok(false),
                    // If the response is anything else, the function returns an error
                    _ => return Err("Unexpected response from server".into()),
                }
            },
            Err(e) => {
                // Error occurred, possibly a network issue or server error, wait before trying again
                eprintln!("Error checking turn: {}, will retry after {:?} seconds", e, delay);
                sleep(delay);
                delay = std::cmp::min(delay.saturating_mul(2), Duration::from_secs(10));
            }
        }
    }
}

// Function to get the current game state
// This function makes a GET request to the server to get the current game state
// The game state is parsed from the response and returned
fn get_game_state() -> State {
    let mut delay = Duration::from_secs(3);
    loop {
        match get_json() {
            Ok(resp) => return parse_state(resp.into_json().expect("Error parsing response to json")),
            Err(_e) => {
                sleep(delay);
                delay *= 2;
                delay = std::cmp::min(Duration::from_millis(10000), delay);
            },
        }
    }
}

// This function makes a GET request to the server to get the current game board
// The response is returned as a Result
fn get_json() -> Result<Response, ureq::Error> {
    let url = format!("{}/board", SERVER_URL);
    let resp = ureq::get(&url).call()?;
    Ok(resp)
}

// Function to send the AI's move to the server
fn send_move(player: &String, ai_move: Option<Action>) -> Result<Response, ureq::Error> {
    let resp;
    let url;
    // If the AI has a move, format the URL for the setChoice endpoint
    // The setChoice endpoint requires the x and y coordinates of the move and the player
    if ai_move.is_some() {
        let ai_choice = ai_move.unwrap();
        url =  format!("{}/setChoice/{}/{}/{}",SERVER_URL, ai_choice.x, ai_choice.y, player);
    }
    // If the AI does not have a move, format the URL for the skipTurn endpoint
    // The skipTurn endpoint requires the player
    else {
        url = format!("{}/skipTurn/{}", SERVER_URL, player);
    }
    resp = ureq::get(&url).call()?;
    Ok(resp)
}

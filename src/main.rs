use ureq::Response;
use std::thread::sleep;
use std::time::Duration;
mod mcts;
mod othello;
use mcts::{MCTS, Node};
use othello::{State, Action, parse_state};



const SERVER_URL: &str = "http://localhost:8181";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let ai_color;
    match args.get(1).expect("Please specify color to the AI").to_lowercase() {
        x if x == "false" => ai_color = 'B',
        x if x == "0" => ai_color = 'B',
        x if x == "b" => ai_color = 'B',
        x if x == "black" => ai_color = 'B',
        x if x == "true" => ai_color = 'W',
        x if x == "1" => ai_color = 'W',
        x if x == "w" => ai_color = 'W',
        x if x == "white" => ai_color = 'W',
        _ => panic!("Please pass a proper argument to the AI"),
        
    }
    let mut state = State::new();
    let mut mcts = MCTS::new(Node::new(state, None, state.get_actions()));
    let mut choice: Result<Action, ()>;
    loop {
        match is_my_turn(ai_color) {
            Ok(true) =>  {
                state = get_game_state();
                choice = mcts.search(state, 10000);

                if choice.is_ok() {
                    let _ = send_move(ai_color, Some(choice.clone().unwrap()));
                    state.do_action(Some(choice.unwrap()));
                }
                else {
                    let _ = send_move(ai_color, None);
                    state.do_action(None); 
                }
                _ = mcts.search(state, 20000);

            },
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

fn is_my_turn(player: char) -> Result<bool, Box<dyn std::error::Error>> {
    let mut delay = Duration::from_secs(1);
    let color = match player {
        'W' => "true",
        _ => "false"
    };
    loop {
        let url = format!("{}/turn/{}", SERVER_URL, color);
        match ureq::get(&url).call() {
            Ok(response) => {
                let body = response.into_string()?;
                match body.trim() {
                    x if x == "true" => return Ok(true),
                    x if x == "false" => { return Ok(false)
                        // It's not our turn yet, wait before trying again
                        //println!("Not my turn. Waiting...");
                        //sleep(delay);
                        //delay = std::cmp::min(delay.saturating_mul(2), Duration::from_secs(3));
                    },
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

fn get_json() -> Result<Response, ureq::Error> {
    let url = format!("{}/board", SERVER_URL);
    let resp = ureq::get(&url).call()?;
    Ok(resp)
}

fn send_move(color: char, ai_move: Option<Action>) -> Result<Response, ureq::Error> {
    let resp;
    let url;
    let player = match color {
        'W' => "true",
        _ => "false",
    };
   if ai_move.is_some() {
        let ai_choice = ai_move.unwrap();
        url =  format!("{}/setChoice/{}/{}/{}",SERVER_URL, ai_choice.x, ai_choice.y, player);
    }
    else {
        url = format!("{}/skipTurn/{}", SERVER_URL, player);
    }
    resp = ureq::get(&url).call()?;
    Ok(resp)
}

use ureq::Response;
use std::thread::sleep;
use std::time::Duration;
mod mcts;
mod othello;
use mcts::{MCTS, Node};
use othello::{State, Action, parse_state};



const SERVER_URL: &str = "http://localhost:8181";

fn main() {
    let mut state = State::new();
    let mut mcts = MCTS::new(Node::new(state, None, state.get_actions()));
    let mut choice: Result<Action, ()>;
    loop {
        match is_my_turn() {
            Ok(true) =>  {
                state = get_game_state();
                choice = mcts.search(state, 10000);

                if choice.is_ok() {
                    let _ = send_move(Some(choice.clone().unwrap()));
                    state.do_action(Some(choice.unwrap()));
                }
                else {
                    let _ = send_move(None);
                    state.do_action(None); 
                }
                _ = mcts.search(state, 20000);

            },
            Ok(false) => {
                sleep(Duration::from_secs(1));
                continue;
            },
            Err(e) => {
                eprintln!("Error checking turn: {}", e);
                sleep(Duration::from_secs(1));
                continue;
            }
        }
    }
}

fn is_my_turn() -> Result<bool, Box<dyn std::error::Error>> {
    let mut delay = Duration::from_secs(1);
    loop {
        let url = format!("{}/turn", SERVER_URL);
        match ureq::get(&url).call() {
            Ok(response) => {
                let body = response.into_string()?;
                match body.trim() {
                    "true" => return Ok(true),
                    "false" => {
                        // It's not our turn yet, wait before trying again
                        println!("Not my turn. Waiting...");
                        sleep(delay);
                        delay = std::cmp::min(delay.saturating_mul(2), Duration::from_secs(3));
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

fn send_move(ai_move: Option<Action>) -> Result<Response, ureq::Error> {
    let resp;
    let base_url = SERVER_URL.to_string();
    let url;
    if ai_move.is_some() {
        let ai_choice = ai_move.unwrap();
        url =  format!("{}/{}/{}/{}/{}",base_url, "setChoice", ai_choice.x, ai_choice.y, true);
    }
    else {
        url = format!("{}/{}", base_url, "skipTurn");
    }
    resp = ureq::get(&url).call()?;
    Ok(resp)
}

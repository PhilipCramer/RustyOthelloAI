use ureq::Response;
use std::thread::sleep;
use std::time::Duration;
mod mcts;
mod othello;
use mcts::{MCTS, Node};
use othello::{State, Action, parse_state};



const SERVER_URL: &str = "http://127.0.0.1:8080";

fn main() {
    let start_state = State::new();
    let mut mcts = MCTS::new(Node::new(start_state, None, start_state.get_actions()));
    loop {
        let current_state = get_game_state();
        let choice = mcts.search(current_state, 10000);
        if choice.is_ok() {
            let _ = send_move(Some(choice.unwrap()));
        }
        else {
            let _ = send_move(None);
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
        url =  format!("{}/{}/{}/{}",base_url, "send", ai_choice.x, ai_choice.y);
    }
    else {
        url = format!("{}/{}", base_url, "skip");
    }
    resp = ureq::post(&url).call()?;
    Ok(resp)
}

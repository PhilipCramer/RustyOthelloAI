use ureq::{json, Response};
use std::thread::{self, sleep};
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


    /*/======== This block is just for testing ========================
    let response = get_game_state();
    println!("Received userId: {}", response["userId"]);
    let res = send_move("b6").unwrap();
    println!("Response status: {} {}", res.status(), res.status_text());


    let test_state = State::new();
    let mut mcts = MCTS::new(Node::new(test_state, None, test_state.get_actions()));
    sleep(Duration::from_secs(3));
    println!("Starting search!");
    println!("Best action: {:?}", mcts.search(test_state, 100000));


    //======== This block is just for testing ========================*/
}




fn get_game_state() -> State {
    let mut delay = Duration::from_millis(10);
     loop {

        match get_json() {
            Ok(resp) => return parse_state(resp.into_json().expect("Error parsing response to json")),
            Err(_e) => {
                thread::sleep(delay);
                delay *= 2;
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

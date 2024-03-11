use ureq::{json, Response};
use std::thread::{self, sleep};
use std::time::Duration;
mod mcts;
mod othello;
use mcts::{MCTS, Node};
use othello::State;



fn main() {

    //======== This block is just for testing ========================
    let response = get_game_state();
    println!("Received userId: {}", response["userId"]);
    let res = send_move("b6").unwrap();
    println!("Response status: {} {}", res.status(), res.status_text());


    let test_state = State::new();
    let mut mcts = MCTS::new(Node::new(test_state, None, test_state.get_actions()));
    sleep(Duration::from_secs(3));
    println!("Starting search!");
    println!("Best action: {:?}", mcts.search(test_state, 100000));


    //======== This block is just for testing ========================
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
    let url = "http://jsonplaceholder.typicode.com/posts";
    let json_body = json!({
        "title": ai_move
    });
    let resp = ureq::post(url)
        .send_json(json_body)?;
    Ok(resp)
}

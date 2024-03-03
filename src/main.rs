use std::thread;
use std::time::Duration;


fn main() {
    let response = get_game_state();
    println!("{:?}", response);

}

fn get_game_state() -> serde_json::Value {
    let url = "http://jsonplaceholder.typicode.com/users/3/address";
    let mut delay = Duration::from_millis(10);
    loop {

        match get_json(url) {
            Ok(json) => return json,
            Err(_e) => {
                thread::sleep(delay);
                delay *= 2;
            },
        }
    }
}

fn get_json(url: &str) -> Result<serde_json::Value, ureq::Error> {
    let resp: serde_json::Value = ureq::get(url)
        .call()?
        .into_json()?;
    println!("{}", resp);
    Ok(resp)
}

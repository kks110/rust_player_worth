use std::env;
use serde_json::Value;

#[tokio::main]
async fn main() {



    match untyped_example().await {
        Some(s) => print!("{}", s),
        None => {}
    }
}

fn api_key() -> String {
    dotenv::dotenv().ok();

    env::var("RIOT_API_KEY")
        .expect("Expected an api key in the environment")
}

async fn untyped_example() -> Option<String> {
    let request_url = format!("https://euw1.api.riotgames.com/lol/summoner/v4/summoners/by-name/{player}?api_key={key}",
                              player = "kks110",
                              key = api_key());
    let response = match reqwest::get(&request_url).await {
        Ok(res) => res,
        Err(_) => return None
    };

    let response_string = match response.text().await {
        Ok(resp_str) => resp_str,
        Err(_) => return None
    };

    // Parse the string of data into serde_json::Value.
    let v: Value = match serde_json::from_str(&response_string) {
        Ok(value) => value,
        Err(_) => return None
    };

    // Access parts of the data by indexing with square brackets.
    println!("{}", v);
    println!("{}", v["name"]);

    return Some(String::from("yay"))
}

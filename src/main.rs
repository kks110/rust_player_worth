use std::env;
use serde_json::Value;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let api_key = env::var("RIOT_API_KEY")
        .expect("Expected a token in the environment");


    match untyped_example(&api_key).await {
        Some(s) => print!("{}", s),
        None => {}
    }
}


async fn untyped_example(api_key: &str) -> Option<String> {
    let request_url = format!("https://euw1.api.riotgames.com/lol/summoner/v4/summoners/by-name/{player}?api_key={key}",
                              player = "kks110",
                              key = api_key);
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

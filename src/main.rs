#[macro_use] extern crate rocket;

use std::time::Duration;

use reqwest::Url;
use rocket::request::Form;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use tokio::time::delay_for;

#[derive(FromForm)]
struct Challenge {
    #[form(field = "hub.challenge")]
    challenge: Option<String>,
    #[form(field = "hub.lease_seconds")]
    lease_seconds: Option<u64>,
    #[form(field = "hub.mode")]
    mode: String,
    #[form(field = "hub.topic")]
    topic: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    data: Vec<EventType>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum EventType {
    StreamEvent(StreamEvent),
    FollowEvent(FollowEvent),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamEvent {
    pub id: String,
    pub user_id: String,
    pub user_name: String,
    pub game_id: Option<String>,
    pub community_ids: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub stream_type: String,
    pub title: String,
    pub viewer_count: u64,
    started_at: String,
    language: String,
    thumbnail_url: String
}
#[derive(Debug, Serialize, Deserialize)]
struct FollowEvent {
    from_id: String,
    from_name: String,
    to_id: String,
    to_name: String,
    followed_at: String
}

#[get("/?<query..>")]
async fn challenge_check(query: Form<Challenge>) -> String {
    println!("Incoming GET request on /");

    let res = if let Some(challenge) = &query.challenge {
        challenge
    } else {
        "Invalid request!"
    };

    res.to_owned()

}

#[post("/", data = "<data>", format = "json")]
async fn incoming_event(data: Json<Data>) {
    println!("New POST request on /");
}

#[launch]
async fn rocket() -> rocket::Rocket {
    let rocket = rocket::ignite()
        .mount("/", routes![challenge_check, incoming_event]);

    rocket::tokio::spawn(async move {
        let _ = init().await;
    });

    rocket
}

async fn init() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    delay_for(Duration::from_secs(3)).await;

    loop {
        let url = Url::parse_with_params("https://api.twitch.tv/helix/webhooks/hub", 
        &[
                ("hub.callback", "example callback url"),
                ("hub.lease_seconds", "10000"),
                ("hub.topic", "example subscription URL"),
                ("hub.mode", "subscribe")
            ])?;

        client.post(url)
            .header("Client-Id", "example client id")
            .header("Authorization", "example token")
            .send().await?;

        delay_for(Duration::from_secs(86400)).await;
    }
}

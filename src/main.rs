use rocket::request::Form;

#[macro_use] extern crate rocket;

#[derive(FromForm)]
struct Challenge {
    #[form(field = "hub.challenge")]
    challenge: String,
    #[form(field = "hub.lease_seconds")]
    lease_seconds: Option<u64>,
    #[form(field = "hub.mode")]
    mode: String,
    #[form(field = "hub.topic")]
    topic: String
}

#[get("/?<query..>")]
async fn challenge_check(query: Form<Challenge>) -> String {
    println!("Incoming GET request on /");

    query.challenge.clone()
}

#[launch]
async fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![challenge_check])
}

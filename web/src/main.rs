#![feature(plugin)]
#![plugin(rocket_codegen)]
use std::io::prelude::*;
use rocket::local::Client;
extern crate rocket;
extern crate slack_hook;
use slack_hook::{Slack, PayloadBuilder};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn client_post() {
    let rocket = rocket::ignite();
    let client = Client::new(rocket).expect("valid rocket");
    let response = client.post("/")
        .body("Hello, world!")
        .dispatch();
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
    let slack = Slack::new("https://hooks.slack.com/services/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").unwrap();
    let p = PayloadBuilder::new()
        .test("JIMMY SUGER")
        .username("TrashTalker")
        .build()
        .unwrap();

    let res = slack.send(&p);
//    match  {
//        Ok(()) => println!("OK"),
//        Err(x) => println!("ERR: {:?}",x)
//    }
}
#![feature(plugin)]
#![plugin(rocket_codegen)]
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use rocket::local::Client;
extern crate rocket;

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
}
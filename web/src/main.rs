#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate slack_hook;
use slack_hook::{Slack, PayloadBuilder};



#[get("/")]
fn index() {
    "Hello, world!";
    slack_post()
}

fn slack_post() {
    let slack = Slack::new("https://hooks.slack.com/services/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").unwrap();
    let p = PayloadBuilder::new()
        .text("JIMMY SUGER")
        .username("TrashTalker")
        .build()
        .unwrap();
    let _res = slack.send(&p);
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();

//    match  {
//        Ok(()) => println!("OK"),
//        Err(x) => println!("ERR: {:?}",x)
//    }
}
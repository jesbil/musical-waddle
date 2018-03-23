#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate slack_hook;
extern crate markov;
use markov::markovchain::MarkovChain;
use slack_hook::{Slack, PayloadBuilder};

static mut MARKOV: MarkovChain = MarkovChain::new();


#[get("/trash")]
fn get_trash() {
    "Hello, world!";
    slack_post()
}

#[post("/trash", data="<input>")]
fn post_trash(input: String) {
    let trash: &str = &input;
    save_trash(trash);
}

fn save_trash(trash: &str) {
    println!("Trashtalk {:?}", trash);
    MarkovChain::add_sentence(&mut MARKOV, trash)

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
    rocket::ignite().mount("/", routes![get_trash, post_trash]).launch();

//    match  {
//        Ok(()) => println!("OK"),
//        Err(x) => println!("ERR: {:?}",x)
//    }
}
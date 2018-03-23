#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate slack_hook;
extern crate markov;
use markov::markovchain::MarkovChain;
use markov::file_parser::parse_file;
use rocket::State;
use slack_hook::{Slack, PayloadBuilder};
use std::sync::RwLock;


#[post("/trashtalk", data="<input>")]
fn get_trash(chain: State<RwLock<MarkovChain>>, slack: State<Slack>, input: String) {
    let chain = chain.read().unwrap();
    let message = chain.generate_sentence();
    let p = PayloadBuilder::new()
        .text(message)
        .username("TrashTalker")
        .build()
        .unwrap();
    let _res = slack.send(&p);
}


#[post("/trash", data="<input>")]
fn post_trash(chain: State<RwLock<MarkovChain>>, input: String) {
    let mut chain = chain.write().unwrap();
    chain.add_sentence(input);
}

fn main() {
    let contents = parse_file("markov/resources/rude.txt");
    let splitted = contents.split("\n");
    let sentences = splitted.collect::<Vec<&str>>();
    let mut tt = MarkovChain::new();
    for sentence in sentences {
        tt.add_sentence(sentence.to_string());
    }

    let slack = Slack::new("https://hooks.slack.com/services/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").unwrap();

    rocket::ignite()
        .mount("/", routes![post_trash, get_trash])
        .manage(RwLock::new(tt))
        .manage(slack)
        .launch();
}
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate slack_hook;
extern crate markov;
use markov::markovchain::MarkovChain;
use markov::file_parser::parse_file;
use rocket::State;
use slack_hook::{Slack, PayloadBuilder};

//static mut TRASH_TALKER: TrashTalker = TrashTalker::new();

struct TrashTalker {
    chain: MarkovChain,
    slack: Slack,
}

impl TrashTalker {
    pub fn new() -> TrashTalker {
        TrashTalker {
            chain: MarkovChain::new(),
            slack: Slack::new("https://hooks.slack.com/services/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").unwrap(),
        }
    }

    pub fn generate_trash(&self) -> String {
        let message = self.chain.generate_sentence();
        message
    }

    pub fn save_trash(&mut self, trash: &str) {
        println!("Trashtalk {:?}", trash);
        self.chain.add_sentence(trash);
    }

    pub fn slack_post(&self, message: String) {
        let p = PayloadBuilder::new()
            .text(message)
            .username("TrashTalker")
            .build()
            .unwrap();
        let _res = &self.slack.send(&p);
    }
}

#[get("/trash")]
fn get_trash(state: State<&TrashTalker>){
    let message = state.generate_trash();
    state.slack_post(message);
}

#[post("/trash", data="<input>")]
fn post_trash(state: State<&TrashTalker>,input: String) {
    let trash: &str = &input;
    state.save_trash(trash);
}

fn main() {
    let contents = parse_file("markov/resources/rude.txt");
    let splitted = contents.split("\n");
    let sentences = splitted.collect::<Vec<&str>>();
    let mut tt = TrashTalker::new();
    for sentence in sentences {
        tt.save_trash(sentence);
    }

    rocket::ignite().mount("/", routes![get_trash, post_trash]).manage(tt).launch();
}
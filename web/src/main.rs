#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate slack_hook;
extern crate markov;
use markov::markovchain::MarkovChain;
use markov::file_parser::parse_file;
use slack_hook::{Slack, PayloadBuilder};

static mut TRASH_TALKER: TrashTalker = TrashTalker::new();

struct TrashTalker {
    chain: MarkovChain,
    slack: Slack
}

impl TrashTalker {
    pub fn new() -> TrashTalker {
        TrashTalker {
            chain: MarkovChain::new(),
            slack: Slack::new("https://hooks.slack.com/services/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").unwrap()
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
fn get_trash() {
    let message = TRASH_TALKER.generate_trash();
    TRASH_TALKER.slack_post(message);
}

#[post("/trash", data="<input>")]
fn post_trash(input: String) {
    let trash: &str = &input;
    TRASH_TALKER.save_trash(trash);
}

fn main() {
    let contents = parse_file("markov/resources/rude.txt");
    let splitted = contents.split("\n");
    let sentences = splitted.collect::<Vec<&str>>();

    for sentence in sentences {
        TRASH_TALKER.save_trash(sentence);
    }

    rocket::ignite().mount("/", routes![get_trash, post_trash]).launch();
}
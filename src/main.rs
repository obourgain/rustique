#![allow(unused_imports)]
#[macro_use]
extern crate serde_derive;

extern crate serde_json;

extern crate iron;
extern crate router;
extern crate time;

use std::sync::Arc;
use std::ops::Deref;

use std::collections::HashMap;

use iron::prelude::*;
use iron::status;
use router::Router;
use serde_json::value::ToJson;

fn main() {
    let olivier = Speaker::new("Olivier".to_string(), "Bourgain".to_string());
    let jb = Speaker::new("jb".to_string(), "Petit".to_string());
    let talk = Talk { name: "rust".to_string(), speakers: vec!(olivier, jb) };

    let mut talks = HashMap::new();
    talks.insert("rust".to_string(), talk);

    let arc_talks = Arc::new(talks);

    let mut router = Router::new();
    router.get("/", move |req: &mut Request| hello_talk(req, &arc_talks.clone()), "my-route");

    Iron::new(router).http("localhost:8080").unwrap();
}

fn hello_talk(req: &mut Request, talks: &HashMap<String, Talk>) -> IronResult<Response> {
    let json = talks.get(&"rust".to_string()).unwrap().to_json();
    match json {
        Ok(json) => Ok(Response::with((status::Ok, json.to_string()))),
        Err(msg) => panic!("{}", msg)
    }

}

#[derive(Debug, Serialize)]
struct Speaker {
    firstname: String,
    lastname: String,
}

impl Speaker {
    fn new(firstname: String, lastname: String) -> Speaker {
        Speaker { firstname: firstname, lastname: lastname }
    }
}

#[derive(Debug, Serialize)]
struct Talk {
    name: String,
    speakers: Vec<Speaker>
}

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

extern crate iron;
extern crate time;

use iron::prelude::*;

#[derive(Serialize)]
struct Talk<'t> {
    title: &'t str,
    speakers: Vec<&'t Speaker<'t>>
}

#[derive(Serialize)]
struct Speaker<'s> {
    fisrtname: &'s str,
    lastname: &'s str
}

//impl <'s>Drop for Speaker<'s> {
//    fn drop(&mut self) {
//        println!("dropping {} {}", self.fisrtname, self.lastname);
//    }
//}
//
//impl <'t> Drop for Talk<'t> {
//    fn drop(&mut self) {
//        println!("dropping {}", self.title);
//    }
//}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    let jb = Speaker { fisrtname: "Jean-Baptiste", lastname: "Petit" };
    let ob = Speaker { fisrtname: "Olivier", lastname: "Bourgain" };
    let talk = Talk {title: "Introduction a Rust", speakers: vec![
        &jb, &ob
    ]};
    let option = serde_json::to_string(&talk);
    return match option {
        Ok(s) => Ok(Response::with((iron::status::Ok, s))),
        Err => Err(IronError::new())
    };
}

fn main() {
    let mut chain = Chain::new(hello_world);
    Iron::new(chain).http("localhost:3000").unwrap();
}

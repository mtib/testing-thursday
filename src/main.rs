#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use std::{ops::Deref, sync::atomic::AtomicUsize};

use dotenv::dotenv;
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Participation {
    counter: AtomicUsize,
}

#[get("/")]
fn index(part: State<Participation>) -> Template {
    part.counter
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    Template::render("index", part.deref())
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .manage(Participation {
            counter: AtomicUsize::new(0),
        })
        .mount("/", routes![index])
        .mount("/static", StaticFiles::from("./static"))
        .attach(Template::fairing())
        .launch();
}

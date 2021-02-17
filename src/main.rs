#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::request::Form;
use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;
use std::error::Error;
// use std::io::Result;
// use rocket_contrib::databases::{r2d2, DbError, DatabaseConfig, Poolable};
use sonic_channel::*;

// impl Poolable for sonic_channel::ControlChannel {}

// TODO: pool with r2d2
#[get("/search?<zip>")]
fn search(zip: u64) -> Result<NamedFile, Box<dyn Error>> {
    let channel = SearchChannel::start("localhost:1491", "password")?;
    println!("{}", channel.ping()?);
    channel.quit()?;
    println!("{}", zip);
    Ok(NamedFile::open("public/results.html")?)
}

// #[derive(FromForm)]
// struct Task {
//     complete: bool,
//     description: String,
// }

// #[post("/todo", data = "<task>")]
// fn new(task: Form<Task>) { /* .. */ }

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("public"))
        .mount("/", routes![search])
        .launch();
}

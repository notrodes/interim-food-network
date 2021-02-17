#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket_contrib::serve::StaticFiles;
use rocket::request::Form;
// use rocket_contrib::databases::{r2d2, DbError, DatabaseConfig, Poolable};
use sonic_channel::*;

// impl Poolable for sonic_channel::ControlChannel {}

#[derive(FromForm)]
struct SearchQuery{
    zip: usize
}

#[get("/search?<zip>")]
fn search(zip: usize) {
    println!("{}", zip);
    // ""
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

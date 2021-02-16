#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket_contrib::serve::StaticFiles;
// use rocket_contrib::databases::{r2d2, DbError, DatabaseConfig, Poolable};
use sonic_channel::*;

// impl Poolable for sonic_channel::ControlChannel {}

#[get("/search")]
fn search() -> &'static str {
    ""
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("public"))
        .mount("/", routes![search])
        .launch();
}

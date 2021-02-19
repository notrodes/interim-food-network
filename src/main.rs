#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::{http::Status, request::Form, response::NamedFile};
use rocket_contrib::databases::rusqlite;
use rocket_contrib::serve::StaticFiles;
// use diesel::prelude::*;
use std::error::Error;

use sonic_channel::*;


#[get("/teapot")]
fn teapot() -> Status {
    Status::ImATeapot
}

// TODO: pool with r2d2
#[get("/search?<zip>")]
fn search(zip: u64) -> Result<NamedFile, Box<dyn Error>> {
    let channel = SearchChannel::start("localhost:1491", "password")?;

    channel.quit()?;
    println!("{}", zip);
    Ok(NamedFile::open("public/results.html")?)
}

#[derive(FromForm, Debug)]
struct FormListing {
    test: String,
}

#[post("/create.html", data = "<listing>")]
fn create(conn: MyDatabase, listing: Form<FormListing>) -> Status {
    // conn.get_one();
    // Listings::belonging_to(0).load::<Listing>(conn);
    Status::Ok
}

#[database("my_db")]
struct MyDatabase(rusqlite::Connection);

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("public"))
        .mount("/", routes![search, create, teapot])
        .attach(MyDatabase::fairing())
        .launch();
}

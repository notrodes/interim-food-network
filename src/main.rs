#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::{http::Status, request::Form};
use rocket_contrib::{ databases::rusqlite, serve::StaticFiles, templates::Template};
use serde::Serialize;
use std::error::Error;

// use sonic_channel::*;

#[get("/teapot")]
fn teapot() -> Status {
    Status::ImATeapot
}

#[get("/search?<zip>")]
fn search(conn: MyDatabase, zip: u32) -> Result<Template, Box<dyn Error>> {
    // let channel = SearchChannel::start("localhost:1491", "password")?;

    // channel.quit()?;
    let mut query = conn.prepare("SELECT * from listings where zip=?")?;
    let results: Vec<Listing> = query
        .query_map(&[&zip], |row| row.get(2))?
        .map(|name| Listing {
            zip,
            name: name.unwrap(),
        })
        .collect();
    let serialized = json!({ "listings": &results, "zip": zip });
    Ok(Template::render("results", serialized))
}

#[derive(Serialize, FromForm, Debug)]
struct Listing {
    zip: u32,
    name: String,
}

#[post("/create.html", data = "<listing>")]
fn create(conn: MyDatabase, listing: Form<Listing>) -> rusqlite::Result<Status> {
    // If there is a table with the zip code
    conn.execute(
        "INSERT INTO listings (zip, name) VALUES (?1, ?2)",
        &[&listing.zip, &listing.name],
    )?;
    Ok(Status::Ok)
}

#[database("my_db")]
struct MyDatabase(rusqlite::Connection);

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("public"))
        .mount("/", routes![search, create, teapot])
        .attach(MyDatabase::fairing())
        .attach(Template::fairing())
        .launch();
}

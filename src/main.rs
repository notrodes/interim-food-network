#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::{http::Status, request::Form, response::NamedFile};
use rocket_contrib::databases::rusqlite;
use rocket_contrib::templates::Template;
// use rocket_contrib::databases::rusqlite:;
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
fn search(conn: MyDatabase, zip: u32) -> Result<Template, Box<dyn Error>> {
    // let channel = SearchChannel::start("localhost:1491", "password")?;

    // channel.quit()?;
    let mut query = conn.prepare("SELECT * from listings where zip=?")?;
    let results = query.query_map(&[&zip], |row| row.get(1)?)?;

    // println!("{}", results);
    Ok(Template::render("public/results.hbs", results))
}

#[derive(FromForm, Debug)]
struct FormListing {
    zip: String,
    name: String,
}

#[post("/create.html", data = "<listing>")]
fn create(conn: MyDatabase, listing: Form<FormListing>) -> rusqlite::Result<Status> {
    // conn.get_one();
    // If there is a table with the zip code
    // panic!();
    // println!("{:?}", conn
    // .prepare("SELECT * FROM zip_codes WHERE code = (?1)")? 
    // .exists(&[&listing.zip]) );
    println!("{:?}", listing);
    
        // panic!("1");
        conn.execute(
            "INSERT INTO listings (zip, name) VALUES (?1, ?2)",
            &[&listing.zip, &listing.name],
        )?;
    // println!("{:?}", );
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

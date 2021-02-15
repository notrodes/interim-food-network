// #![feature(proc_macro_hygiene, decl_macro)]
use rocket;
use rocket_contrib::serve::StaticFiles;

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("public"))
        .launch();
}

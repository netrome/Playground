#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String {
    String::from("Hello, derpface!")
}

#[get("/ruth")]
fn ruth() -> String {
    String::from("Hello, sisukkej!")
}

fn main() {
    rocket::ignite().mount("/", routes![index, ruth]).launch();
}

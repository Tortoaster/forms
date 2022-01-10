#[macro_use]
extern crate rocket;

use rocket::error::Error;

use itasks::prelude::*;
use itasks::task::Action;

#[derive(Component)]
struct Person {
    name: String,
    age: u8,
    cool: bool,
}

#[get("/")]
fn index() -> Task<Person> {
    enter()
}

#[get("/test")]
fn test() -> Task<Person> {
    view("Welcome!")
        .actions()
        .on(Action::Ok, |_| index())
        .finalize()
}

#[rocket::main]
async fn main() -> Result<(), Box<Error>> {
    rocket::build()
        .mount("/", routes![index, test])
        .launch()
        .await?;

    Ok(())
}

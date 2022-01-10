#[macro_use]
extern crate rocket;

use rocket::error::Error;

use itasks::prelude::*;
use itasks::task::Action;

#[derive(Component)]
struct Person {
    name: String,
    age: u8,
}

#[get("/")]
fn index() -> Task<Person> {
    let person = Person {
        name: "Rick".to_owned(),
        age: 22,
    };

    view(person)
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

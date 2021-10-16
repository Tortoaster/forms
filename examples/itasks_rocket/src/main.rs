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
    enter()
        .actions()
        .on(Action::Ok, |name| enter()
            .actions()
            .on(Action::Ok, |age| view(Person { name, age }))
            .finalize()
        )
        .finalize()
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

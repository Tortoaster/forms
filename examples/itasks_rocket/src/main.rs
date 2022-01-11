#[macro_use]
extern crate rocket;

use rocket::error::Error;

use itasks::prelude::*;

#[derive(Component)]
struct Person {
    name: String,
    age: u8,
    cool: bool,
}

#[derive(Component)]
struct People {
    person: Person,
    lol: bool,
}

#[get("/")]
fn index() -> Task<People> {
    enter()
}

// #[get("/test")]
// fn test() -> Task<Person> {
//     view("Welcome!")
//         .actions()
//         .on(Action::Ok, |_| index())
//         .finalize()
// }

#[rocket::main]
async fn main() -> Result<(), Box<Error>> {
    rocket::build()
        .mount("/", routes![index])
        .attach(itasks::backend::ITasks)
        .launch()
        .await?;

    Ok(())
}

#[macro_use]
extern crate rocket;

use rocket::error::Error;

use itasks::prelude::*;

#[derive(Component)]
struct Person {
    name: String,
    age: u8,
}

#[get("/")]
async fn index() -> Task<(Person, Person)> {
    let me = Person {
        name: "Rick".to_owned(),
        age: 22,
    };

    enter().and(view(me))
}

#[rocket::main]
async fn main() -> Result<(), Box<Error>> {
    rocket::build()
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}

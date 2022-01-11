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

#[get("/")]
fn index() -> Task<(Person, Person)> {
    let me = Person {
        name: "Rick".to_string(),
        age: 22,
        cool: true,
    };

    enter().and(view(me))
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
    rocket::build().mount("/", routes![index]).launch().await?;

    Ok(())
}

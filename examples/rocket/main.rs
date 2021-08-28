#![cfg(feature = "rocket")]
#[macro_use]
extern crate rocket;

use rocket::error::Error;

use itasks::component::Component;
use itasks_derive::Component;
use itasks::task::{Task, view};

#[derive(Component)]
struct Field(String);

#[derive(Component)]
struct Struct {
    field: Field,
}

#[get("/")]
async fn index() -> Task<Struct> {
    let component = Struct {
        field: Field("Hello".to_owned())
    };

    view(component)
}

#[rocket::main]
async fn main() -> Result<(), Box<Error>> {
    rocket::build()
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}

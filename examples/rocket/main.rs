#![cfg(feature = "rocket")]
#[macro_use]
extern crate rocket;

use rocket::error::Error;

use itasks::component::Component;
use itasks::prelude::*;
use itasks::task::Task;
use itasks_derive::Component;

#[derive(Clone, Component)]
struct Unit;

#[derive(Clone, Component)]
struct Field(String);

#[derive(Clone, Component)]
struct Struct {
    field: Field,
    unit: Unit,
}

#[get("/")]
async fn index() -> Task<Struct> {
    let component = Struct {
        field: Field("Hello".to_owned()),
        unit: Unit,
    };

    view(component.clone()).or(update(component)).or(enter())
}

#[rocket::main]
async fn main() -> Result<(), Box<Error>> {
    rocket::build()
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}

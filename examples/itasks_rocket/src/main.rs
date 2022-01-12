#[macro_use]
extern crate rocket;

use rocket::error::Error;

use itasks::prelude::*;

#[derive(Component)]
struct UnitStruct;

#[derive(Component)]
struct TupleStruct(String, i32, bool, UnitStruct);

#[derive(Component)]
struct NamedStruct {
    name: String,
    number: i32,
    works: bool,
    tuple_struct: TupleStruct,
}

#[get("/")]
fn index() -> Task<NamedStruct> {
    enter()
}

#[rocket::main]
async fn main() -> Result<(), Box<Error>> {
    rocket::build()
        .mount("/", routes![index])
        .attach(itasks::backend::ITasks)
        .launch()
        .await?;

    Ok(())
}

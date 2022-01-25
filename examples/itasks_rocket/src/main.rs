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

#[derive(Component)]
enum Enum {
    Unit,
    Tuple(u8, bool),
    Named { number: u8, truth: bool },
}

#[get("/")]
fn index() -> Task<Enum> {
    let e = Enum::Named {
        number: 8,
        truth: true,
    };
    update(e)
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

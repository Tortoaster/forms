#[macro_use]
extern crate rocket;

use rocket::error::Error;

use itasks::prelude::*;
use itasks::task::Action;

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
fn index() -> Task<TupleStruct> {
    enter::<String>()
        .actions()
        .on(Action::Ok, |name| {
            let ts = TupleStruct(name, 42, true, UnitStruct);
            view(ts)
        })
        .on(Action::Cancel, |_| {
            view(TupleStruct(":(".to_owned(), 0, false, UnitStruct))
        })
        .finalize()
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

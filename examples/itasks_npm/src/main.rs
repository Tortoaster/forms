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

fn main() -> Task<NamedStruct> {
    enter()
}

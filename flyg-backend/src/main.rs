#![feature(decl_macro, proc_macro_hygiene)]

use rocket::routes;

pub mod routes;

fn main() {
    rocket::ignite()
        .mount(
            "/v1",
            routes![crate::routes::airports::get_airport_information],
        )
        .launch();
}

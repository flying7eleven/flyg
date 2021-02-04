#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;

use rocket_contrib::database;
use rocket_contrib::databases::diesel::PgConnection;

pub mod database;
pub mod routes;

#[database("flyg_database")]
pub struct FlygDatabaseConnection(PgConnection);

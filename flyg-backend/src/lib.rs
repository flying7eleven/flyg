#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;

pub mod database;
pub mod routes;

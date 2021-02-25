#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

use rocket_contrib::database;
use rocket_contrib::databases::diesel::PgConnection;

pub mod database;
pub mod routes;

#[database("flyg_database")]
pub struct FlygDatabaseConnection(PgConnection);

#[derive(Clone)]
pub struct FlygSettings {
    pub database_url: String,
}

pub fn get_configuration() -> FlygSettings {
    use std::fs::File;
    use std::io::{BufReader, Read};

    // try to open the configuration file...
    if let Ok(file) = File::open("Rocket.toml") {
        // ... and read the whole file into the memory
        let mut contents = String::new();
        let mut file_reader = BufReader::new(file);
        if let Ok(_) = file_reader.read_to_string(&mut contents) {
            // parse the read string into a Toml table
            if let Ok(read_values) = toml::from_str::<toml::value::Table>(contents.as_str()) {
                // get the global section
                if let Some(global) = read_values.get("global") {
                    // get the database sub-section
                    if let Some(databases) = global.get("databases") {
                        // get the configuration entry for the database
                        if let Some(pmgdb) = databases.get("flyg_database") {
                            // try to extract the database URL and return it as a settings object
                            if let Some(db_url) = pmgdb.get("url") {
                                return FlygSettings {
                                    database_url: db_url.to_string().replace("\"", ""),
                                };
                            }
                        }
                    }
                }
            }
        }
    }

    // if we reach here, we can terminate since we could not read the required
    // configuration information
    panic!("Failed to read the configuration file. Terminating...");
}

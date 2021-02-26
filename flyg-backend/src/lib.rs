#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;

#[cfg(not(debug_assertions))]
#[macro_use]
extern crate diesel_migrations;

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
    pub private_key: String,
    pub public_key: String,
}

fn read_key(filename: &String) -> Option<String> {
    use log::error;
    use std::fs::File;
    use std::io::{BufReader, Read};

    //
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            error!(
                "Failed to read {} into memory. The error was: {}",
                filename, error
            );
            return None;
        }
    };

    //
    let mut contents = String::new();
    let mut file_reader = BufReader::new(file);
    if let Ok(_) = file_reader.read_to_string(&mut contents) {
        return Some(contents);
    }

    //
    None
}

pub fn get_configuration() -> FlygSettings {
    use std::fs::File;
    use std::io::{BufReader, Read};

    //
    let mut database_url = "".to_string();
    let mut private_key_file = "".to_string();
    let mut public_key_file = "".to_string();

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
                                database_url = db_url.to_string().replace("\"", "");
                            }
                        }
                    }

                    // get the flyg sub-section
                    if let Some(flyg) = global.get("flyg") {
                        // get the token sub-section
                        if let Some(token) = flyg.get("token") {
                            // get the public key file
                            if let Some(public_key) = token.get("public_key") {
                                public_key_file = public_key.to_string().replace("\"", "");
                            }

                            // get the private key file
                            if let Some(private_key) = token.get("private_key") {
                                private_key_file = private_key.to_string().replace("\"", "");
                            }
                        }
                    }
                }
            }
        }
    }

    //
    if !database_url.is_empty() && !public_key_file.is_empty() && !private_key_file.is_empty() {
        //
        let maybe_public_key = read_key(&public_key_file);
        let maybe_private_key = read_key(&private_key_file);

        //
        if maybe_public_key.is_some() && maybe_private_key.is_some() {
            return FlygSettings {
                database_url,
                public_key: maybe_public_key.unwrap(),
                private_key: maybe_private_key.unwrap(),
            };
        }
    }

    // if we reach here, we can terminate since we could not read the required
    // configuration information
    panic!("Failed to read the configuration file. Terminating...");
}

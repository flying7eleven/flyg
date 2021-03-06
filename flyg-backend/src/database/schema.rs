table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    aircraft (id) {
        id -> Int4,
        aircraft_model -> Int4,
        registration -> Varchar,
        owner -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    aircraft_models (id) {
        id -> Int4,
        manufacturer -> Varchar,
        model -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    aircrafts_on_market (id) {
        id -> Int4,
        aircraft_id -> Int4,
        price -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    airports (id) {
        id -> Int4,
        icao_code -> Varchar,
        last_update -> Timestamp,
        country -> Varchar,
        location -> Geography,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    flight_log (id) {
        id -> Int4,
        aircraft_id -> Int4,
        pilot_id -> Int4,
        departure_airport_id -> Int4,
        arrival_airport_id -> Int4,
        date_of_flight -> Varchar,
        off_block_time -> Int4,
        takeoff_time -> Int4,
        landing_time -> Int4,
        on_block_time -> Int4,
        distance -> Nullable<Int4>,
        duration -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    runway_airport_associations (id) {
        id -> Int4,
        airport_id -> Int4,
        runway_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    runways (id) {
        id -> Int4,
        primary_direction -> Int4,
        secondary_direction -> Int4,
        primary_suffix -> Nullable<Varchar>,
        runway_length -> Int4,
        runway_width -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::sql_types::*;

    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email_address -> Varchar,
        password -> Varchar,
        is_admin -> Bool,
    }
}

joinable!(aircraft -> aircraft_models (aircraft_model));
joinable!(aircraft -> users (owner));
joinable!(aircrafts_on_market -> aircraft (aircraft_id));
joinable!(flight_log -> aircraft (aircraft_id));
joinable!(flight_log -> users (pilot_id));
joinable!(runway_airport_associations -> airports (airport_id));
joinable!(runway_airport_associations -> runways (runway_id));

allow_tables_to_appear_in_same_query!(
    aircraft,
    aircraft_models,
    aircrafts_on_market,
    airports,
    flight_log,
    runway_airport_associations,
    runways,
    spatial_ref_sys,
    users,
);

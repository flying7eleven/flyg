table! {
    aircraft (id) {
        id -> Int4,
        aircraft_model -> Int4,
    }
}

table! {
    aircraft_models (id) {
        id -> Int4,
    }
}

table! {
    airports (id) {
        id -> Int4,
        icao_code -> Varchar,
        last_update -> Timestamp,
        country -> Varchar,
        longitude -> Float4,
        latitude -> Float4,
        name -> Varchar,
    }
}

table! {
    runway_airport_associations (id) {
        id -> Int4,
        airport_id -> Int4,
        runway_id -> Int4,
    }
}

table! {
    runways (id) {
        id -> Int4,
        primary_direction -> Int4,
        secondary_direction -> Int4,
        primary_denominator -> Nullable<Varchar>,
        runway_length -> Int4,
        runway_width -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
    }
}

joinable!(aircraft -> aircraft_models (aircraft_model));
joinable!(runway_airport_associations -> airports (airport_id));
joinable!(runway_airport_associations -> runways (runway_id));

allow_tables_to_appear_in_same_query!(
    aircraft,
    aircraft_models,
    airports,
    runway_airport_associations,
    runways,
    users,
);

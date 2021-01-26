table! {
    airports (id) {
        id -> Int4,
        icao_code -> Varchar,
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
        direction_one -> Int4,
        direction_two -> Int4,
        runway_length -> Int4,
        runway_width -> Int4,
    }
}

joinable!(runway_airport_associations -> airports (airport_id));
joinable!(runway_airport_associations -> runways (runway_id));

allow_tables_to_appear_in_same_query!(
    airports,
    runway_airport_associations,
    runways,
);

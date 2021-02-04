table! {
    airports (id) {
        id -> Int4,
        icao_code -> Varchar,
        last_update -> Timestamp,
        country -> Varchar,
        coordinates -> Geography,
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
        primary_suffix -> Nullable<Varchar>,
        runway_length -> Int4,
        runway_width -> Int4,
    }
}

table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

joinable!(runway_airport_associations -> airports (airport_id));
joinable!(runway_airport_associations -> runways (runway_id));

allow_tables_to_appear_in_same_query!(
    airports,
    runway_airport_associations,
    runways,
    spatial_ref_sys,
);

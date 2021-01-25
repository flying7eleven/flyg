table! {
    airports (id) {
        id -> Int4,
        icao_code -> Varchar,
        country -> Varchar,
        longitude -> Float4,
        latitude -> Float4,
        name -> Varchar,
        runway_length -> Int4,
        runway_width -> Int4,
    }
}

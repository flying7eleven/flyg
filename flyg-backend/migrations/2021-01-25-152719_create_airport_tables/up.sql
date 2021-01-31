CREATE TABLE airports
(
    id        SERIAL PRIMARY KEY,
    icao_code VARCHAR(4) NOT NULL UNIQUE, -- the official ICAO code of the airport
    country   VARCHAR(2) NOT NULL,        -- two letter country code
    longitude REAL       NOT NULL,        -- with 6 decimal places
    latitude  REAL       NOT NULL,        -- with 6 decimal places
    name      VARCHAR    NOT NULL         -- in English
);

CREATE TABLE runways
(
    id            SERIAL PRIMARY KEY,
    direction_one INTEGER NOT NULL,
    direction_two INTEGER NOT NULL,
    runway_length INTEGER NOT NULL,
    runway_width  INTEGER NOT NULL
);

CREATE TABLE runway_airport_associations
(
    id         SERIAL PRIMARY KEY, -- requirement for diesel-rs, otherwise not needed
    airport_id INTEGER NOT NULL,
    runway_id  INTEGER NOT NULL,
    CONSTRAINT fk_runway_airport_association_airport_id FOREIGN KEY (airport_id) REFERENCES airports (id),
    CONSTRAINT fk_runway_airport_association_runway_id FOREIGN KEY (runway_id) REFERENCES runways (id)
);
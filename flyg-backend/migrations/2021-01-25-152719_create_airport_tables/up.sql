CREATE TABLE airports
(
    id        SERIAL PRIMARY KEY,
    icao_code VARCHAR(4) NOT NULL UNIQUE, -- the official ICAO code of the airport
    country   VARCHAR(2) NOT NULL,        -- two letter country code
    longitude REAL       NOT NULL,        -- with 6 decimal places
    latitude  REAL       NOT NULL,        -- with 6 decimal places
    name      VARCHAR    NOT NULL         -- in English
);

INSERT INTO airports
VALUES (DEFAULT, 'EDKA', 'DE', 50.823333, 6.186389, 'Aachen Merzbrück Airfield'),
       (DEFAULT, 'EDLN', 'DE', 51.230278, 6.504444, 'Mönchengladbach Airport'),
       (DEFAULT, 'EDRK', 'DE', 50.324739, 7.527305, 'Koblenz-Winningen Airport'),
       (DEFAULT, 'EDLA', 'DE', 51.483333, 7.899333, 'Arnsberg-Menden Airport'),
       (DEFAULT, 'EDDL', 'DE', 51.280925, 6.757311, 'Düsseldorf Airport');

CREATE TABLE runways
(
    id            SERIAL PRIMARY KEY,
    direction_one INTEGER NOT NULL,
    direction_two INTEGER NOT NULL,
    runway_length INTEGER NOT NULL,
    runway_width  INTEGER NOT NULL
);

INSERT INTO runways
VALUES (DEFAULT, 7, 25, 1160, 18),
       (DEFAULT, 13, 31, 1200, 30),
       (DEFAULT, 6, 24, 1175, 20),
       (DEFAULT, 5, 23, 920, 20),
       (DEFAULT, 5, 23, 3000, 45),
       (DEFAULT, 5, 23, 2700, 45);

CREATE TABLE runway_airport_associations
(
    id         SERIAL PRIMARY KEY, -- requirement for diesel-rs, otherwise not needed
    airport_id INTEGER NOT NULL,
    runway_id  INTEGER NOT NULL,
    CONSTRAINT fk_runway_airport_association_airport_id FOREIGN KEY (airport_id) REFERENCES airports (id),
    CONSTRAINT fk_runway_airport_association_runway_id FOREIGN KEY (runway_id) REFERENCES runways (id)
);

INSERT INTO runway_airport_associations
VALUES (DEFAULT, 1, 1),
       (DEFAULT, 2, 2),
       (DEFAULT, 3, 3),
       (DEFAULT, 4, 4),
       (DEFAULT, 5, 5),
       (DEFAULT, 5, 6);
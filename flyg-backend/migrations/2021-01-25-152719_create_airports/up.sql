CREATE TABLE airports
(
    id            SERIAL PRIMARY KEY,
    icao_code     VARCHAR(4) NOT NULL UNIQUE,
    country       VARCHAR(2) NOT NULL,
    longitude     REAL       NOT NULL,
    latitude      REAL       NOT NULL,
    name          VARCHAR    NOT NULL, -- in English
    runway_length INTEGER    NOT NULL, -- in meter (if there are more than one, the length of the longest)
    runway_width  INTEGER    NOT NULL  -- in meter (if there are more than one, the width of the longest)
);

INSERT INTO airports
VALUES (DEFAULT, 'EDKA', 'DE', 50.823333, 6.186389, 'Aachen Merzbrück Airfield', 1160, 18),
       (DEFAULT, 'EDLN', 'DE', 51.230278, 6.504444, 'Mönchengladbach Airport', 1200, 30),
       (DEFAULT, 'EDRK', 'DE', 50.324739, 7.527305, 'Koblenz-Winningen Airport', 1175, 20),
       (DEFAULT, 'EDLA', 'DE', 51.483333, 7.899333, 'Arnsberg-Menden Airport', 920, 20);

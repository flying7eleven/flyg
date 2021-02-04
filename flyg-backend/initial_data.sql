INSERT INTO airports
VALUES (DEFAULT, 'EDKA', '2021-01-31 12:00:00', 'DE', ST_MakePoint(6.186389, 50.823333), 'Aachen Merzbrück Airfield'),
       (DEFAULT, 'EDLN', '2021-01-31 12:00:00', 'DE', ST_MakePoint(6.504444, 51.230278), 'Mönchengladbach Airport'),
       (DEFAULT, 'EDRK', '2021-01-31 12:00:00', 'DE', ST_MakePoint(7.527305, 50.324739), 'Koblenz-Winningen Airport'),
       (DEFAULT, 'EDLA', '2021-01-31 12:00:00', 'DE', ST_MakePoint(7.899333, 51.483333), 'Arnsberg-Menden Airport'),
       (DEFAULT, 'EDDL', '2021-01-31 12:00:00', 'DE', ST_MakePoint(6.757311, 51.280925), 'Düsseldorf Airport');

INSERT INTO runways
VALUES (DEFAULT, 66, 246, DEFAULT, 1160, 18),
       (DEFAULT, 127, 307, DEFAULT, 1200, 30),
       (DEFAULT, 56, 236, DEFAULT, 1175, 20),
       (DEFAULT, 48, 228, DEFAULT, 920, 20),
       (DEFAULT, 52, 232, 'L', 2700, 45),
       (DEFAULT, 52, 232, 'R', 3000, 45);

INSERT INTO runway_airport_associations
VALUES (DEFAULT, 1, 1),
       (DEFAULT, 2, 2),
       (DEFAULT, 3, 3),
       (DEFAULT, 4, 4),
       (DEFAULT, 5, 5),
       (DEFAULT, 5, 6);
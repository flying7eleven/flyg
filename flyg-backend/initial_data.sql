INSERT INTO airports
VALUES (DEFAULT, 'EDKA', 'DE', 50.823333, 6.186389, 'Aachen Merzbrück Airfield'),
       (DEFAULT, 'EDLN', 'DE', 51.230278, 6.504444, 'Mönchengladbach Airport'),
       (DEFAULT, 'EDRK', 'DE', 50.324739, 7.527305, 'Koblenz-Winningen Airport'),
       (DEFAULT, 'EDLA', 'DE', 51.483333, 7.899333, 'Arnsberg-Menden Airport'),
       (DEFAULT, 'EDDL', 'DE', 51.280925, 6.757311, 'Düsseldorf Airport');

INSERT INTO runways
VALUES (DEFAULT, 7, 25, 1160, 18),
       (DEFAULT, 13, 31, 1200, 30),
       (DEFAULT, 6, 24, 1175, 20),
       (DEFAULT, 5, 23, 920, 20),
       (DEFAULT, 5, 23, 3000, 45),
       (DEFAULT, 5, 23, 2700, 45);

INSERT INTO runway_airport_associations
VALUES (DEFAULT, 1, 1),
       (DEFAULT, 2, 2),
       (DEFAULT, 3, 3),
       (DEFAULT, 4, 4),
       (DEFAULT, 5, 5),
       (DEFAULT, 5, 6);
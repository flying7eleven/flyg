-- describes the models of possible aircraft. Each model can be used by different
-- aircraft available on the market
CREATE TABLE aircraft_models
(
    id           SERIAL PRIMARY KEY,
    manufacturer VARCHAR(64) NOT NULL, -- the manufacturer of the plane
    model        VARCHAR(64) NOT NULL, -- the full name of the model of the plane
    UNIQUE (manufacturer, model)
);

--- the actual aircraft on the market which can be brought by users
CREATE TABLE aircraft
(
    id             SERIAL PRIMARY KEY,
    aircraft_model INT               NOT NULL, -- the actual model of the plane
    registration   VARCHAR(7) UNIQUE NOT NULL, -- the registration of the aircraft (e.g. N990C8, D-EABC, etc.)
    owner          INT DEFAULT NULL,           -- is NULL if the aircraft is owned by the system and not an user
    FOREIGN KEY (aircraft_model) REFERENCES aircraft_models (id),
    FOREIGN KEY (owner) REFERENCES users (id)
);
-- the table used for logging flown flights
CREATE TABLE flight_log
(
    id                   SERIAL PRIMARY KEY,
    aircraft_id          INT     NOT NULL,     -- the id of the aircraft which was used for the flight
    pilot_id             INT     NOT NULL,     -- the pilot in command for this flight
    departure_airport_id INT     NOT NULL,     -- the airport from which the plane departed from
    arrival_airport_id   INT     NOT NULL,     -- the airport on which the plane arrived
    date_of_flight       VARCHAR NOT NULL,     -- the date of the departure for the flight (UTC-based) in a format of YYYY-MM-DD
    off_block_time       INT     NOT NULL,     -- the off-block time for the flight (in UTC without separation, e.g. 14:56 is 1456 where as 03:31 would be 331)
    takeoff_time         INT     NOT NULL,     -- the takeoff time for the flight (see example above)
    landing_time         INT     NOT NULL,     -- the landing time for the flight (see example above)
    on_block_time        INT     NOT NULL,     -- the on-block time for the flight (see example above)
    distance             INT     DEFAULT NULL, -- the total (direct-to) distance of the flight in NM (can be NULL if calculated later on),
    duration             VARCHAR DEFAULT NULL, -- the duration of the flight in the format HH:MM (can be NULL if calculated later on)
    CONSTRAINT fk_flight_log_aircraft_id FOREIGN KEY (aircraft_id) REFERENCES aircraft (id),
    CONSTRAINT fk_flight_log_pilot_id FOREIGN KEY (pilot_id) REFERENCES users (id),
    CONSTRAINT fk_flight_log_departure_airport_id FOREIGN KEY (departure_airport_id) REFERENCES airports (id),
    CONSTRAINT fk_flight_log_arrival_airport_id FOREIGN KEY (arrival_airport_id) REFERENCES airports (id)
);

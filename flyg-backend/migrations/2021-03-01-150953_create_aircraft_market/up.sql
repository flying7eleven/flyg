-- a table describing the aircrafts which are on the market for sale (NOT USED RIGHT NOW)
CREATE TABLE aircrafts_on_market
(
    id          SERIAL PRIMARY KEY,
    aircraft_id INT NOT NULL, -- the id of the aircraft which is currently on the market for sale
    price       INT NOT NULL, -- the price of the aircraft in US dollar
    CONSTRAINT fk_aircrafts_on_market_aircraft_id FOREIGN KEY (aircraft_id) REFERENCES aircraft (id)
);
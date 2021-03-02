-- a table which stores all user information for an account. This is the root of
-- all owned aircraft as well as the table which is used for logging in as a
-- specific user
CREATE TABLE users
(
    id            SERIAL PRIMARY KEY,
    first_name    VARCHAR(64)  NOT NULL,
    last_name     VARCHAR(64)  NOT NULL,
    email_address VARCHAR(255) NOT NULL UNIQUE,
    password      VARCHAR(255) NOT NULL,
    is_admin      BOOLEAN NOT NULL DEFAULT FALSE
);
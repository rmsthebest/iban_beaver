-- Your SQL goes here
CREATE TABLE blacklist (
    iban TEXT NOT NULL PRIMARY KEY,
    blacklisted BOOLEAN NOT NULL CHECK (blacklisted IN (0, 1))
)

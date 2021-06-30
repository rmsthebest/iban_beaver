-- Your SQL goes here
-- Unique tables first (only blacklist for now)
-- country tables after. Try to sort alphabetically
CREATE TABLE blacklist (
    iban TEXT NOT NULL PRIMARY KEY,
    blacklisted BOOLEAN NOT NULL CHECK (blacklisted IN (0, 1))
);
CREATE TABLE t_at (
    id INTEGER NOT NULL,
    code TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    zip INTEGER NOT NULL,
    city TEXT NOT NULL,
    bic TEXT
);
CREATE TABLE t_de (
    id INTEGER NOT NULL PRIMARY KEY,
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    zip INTEGER NOT NULL,
    city TEXT NOT NULL,
    bic TEXT
);
CREATE TABLE t_nl (
    code TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    bic TEXT NOT NULL
);

-- Your SQL goes here
CREATE TABLE t_de (
    id INTEGER NOT NULL PRIMARY KEY,
    code INTEGER NOT NULL,
    name TEXT NOT NULL,
    zip INTEGER NOT NULL,
    city TEXT NOT NULL,
    bic TEXT,
    blacklisted BOOLEAN NOT NULL CHECK (blacklisted IN (0, 1))
)
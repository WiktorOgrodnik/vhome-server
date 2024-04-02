-- Add migration script here

DROP TABLE IF EXISTS vlist;

CREATE TABLE vlist (
  id serial PRIMARY KEY,
  name VARCHAR NOT NULL
);

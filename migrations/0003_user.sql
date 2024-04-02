--- Add migration script here

DROP TABLE IF EXISTS vuser;

CREATE TABLE vuser (
  id serial PRIMARY KEY,
  name VARCHAR NOT NULL
);- Add migration script here

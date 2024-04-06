-- Add migration script here

DROP TABLE IF EXISTS vgroup;
DROP TABLE IF EXISTS vlist;
DROP TABLE IF EXISTS participation;

CREATE TABLE vgroup (
  id serial PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE vlist (
  id serial PRIMARY KEY,
  group_id integer NOT NULL references vgroup(id),
  name VARCHAR NOT NULL
);

CREATE TABLE participation (
  id serial PRIMARY KEY,
  name VARCHAR NOT NULL
)

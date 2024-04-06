--- Add migration script here

DROP TABLE IF EXISTS vuser;
DROP TABLE IF EXISTS vuser_groups;

CREATE TABLE vuser (
  id serial PRIMARY KEY,
  login VARCHAR NOT NULL,
  passwd VARCHAR NOT NULL
);

CREATE TABLE vuser_groups (
  user_id integer NOT NULL REFERENCES vuser(id),
  group_id integer NOT NULL REFERENCES vgroup(id),
  participation_id integer NOT NULL REFERENCES participation(id),
  PRIMARY KEY (user_id, group_id, participation_id)
);

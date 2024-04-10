-- Add migration script here

DROP TABLE IF EXISTS vtask;
DROP TABLE IF EXISTS vtask_assign;

CREATE TABLE vtask (
  id serial PRIMARY KEY,
  title VARCHAR NOT NULL,
  content VARCHAR NOT NULL,
  completed BOOLEAN NOT NULL,
  vlist_id INTEGER NOT NULL REFERENCES vlist(id),
  completed_time TIMESTAMPTZ DEFAULT NULL
);

CREATE TABLE vtask_assign (
  vtask_id INTEGER NOT NULL REFERENCES vtask(id),
  vuser_assign INTEGER NOT NULL REFERENCES vuser(id),
  assign_time TIMESTAMPTZ NOT NULL,
  PRIMARY KEY (vtask_id, vuser_assign)
);

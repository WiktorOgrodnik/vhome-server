-- Add migration script here

CREATE TYPE device_type AS ENUM (
  'thermometer',
  'other'
);

CREATE TABLE device (
  id serial PRIMARY KEY,
  group_id INTEGER NOT NULL REFERENCES vgroup(id),
  name VARCHAR NOT NULL,
  dev_t device_type NOT NULL
);

CREATE TABLE thermometer (
  device_id int PRIMARY KEY REFERENCES device(id),
  last_temp real DEFAULT 0,
  last_updated TIMESTAMPTZ DEFAULT NULL
);

INSERT INTO device (group_id, name, dev_t) VALUES
  (1, 'Test thermometer', 'thermometer');

INSERT INTO thermometer (device_id) VALUES (1);

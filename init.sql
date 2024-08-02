CREATE TABLE IF NOT EXISTS vgroup (
  id serial PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS taskset (
  id serial PRIMARY KEY,
  vgroup_id integer NOT NULL references vgroup(id),
  name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS vuser (
  id serial PRIMARY KEY,
  login VARCHAR NOT NULL,
  passwd VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  picutre bytea NOT NULL
);

CREATE TYPE token_type AS ENUM (
  'normal',
  'device'
);

CREATE TABLE IF NOT EXISTS tokens (
  vuser_id integer NOT NULL REFERENCES vuser(id),
  token TEXT NOT NULL,
  token_t token_type NOT NULL,
  PRIMARY KEY (vuser_id, token)
);

CREATE TYPE role_type AS ENUM (
  'guest',
  'member',
  'admin'
);

CREATE TABLE IF NOT EXISTS user_groups (
  vuser_id integer NOT NULL REFERENCES vuser(id),
  vgroup_id integer NOT NULL REFERENCES vgroup(id),
  role role_type NOT NULL,
  PRIMARY KEY (vuser_id, vgroup_id)
);

CREATE TABLE IF NOT EXISTS groups_invitations (
  id serial PRIMARY KEY,
  vgroup_id integer NOT NULL REFERENCES vgroup(id),
  invitation_code VARCHAR NOT NULL,
  expiration_date TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS task (
  id serial PRIMARY KEY,
  title VARCHAR NOT NULL,
  content VARCHAR NOT NULL,
  completed BOOLEAN NOT NULL,
  taskset_id INTEGER NOT NULL REFERENCES taskset(id),
  completed_time TIMESTAMPTZ DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS task_assign (
  task_id INTEGER NOT NULL REFERENCES task(id),
  user_assign INTEGER NOT NULL REFERENCES vuser(id),
  assign_time TIMESTAMPTZ NOT NULL,
  PRIMARY KEY (task_id, user_assign)
);

CREATE TYPE device_type AS ENUM (
  'thermometer',
  'other'
);

CREATE TABLE IF NOT EXISTS device (
  id serial PRIMARY KEY,
  vgroup_id INTEGER NOT NULL REFERENCES vgroup(id),
  name VARCHAR NOT NULL,
  dev_t device_type NOT NULL,
  token Text NOT NULL,
  initialized BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS thermometer (
  device_id int PRIMARY KEY REFERENCES device(id),
  last_temp real,
  last_humidity real,
  last_updated TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

INSERT INTO vgroup (name)
VALUES
  ( 'Friends' ),
  ( 'Family' );

INSERT INTO taskset (vgroup_id, name)
VALUES
  ( 1, 'Places to visit' ),
  ( 1, 'Party members' ),
  ( 2, 'Shopping list' );

INSERT INTO vuser (login, passwd, created_at, picutre)
VALUES
  ( 'Wiktor',  '$2b$12$iLmS6/.s.PrXYuSAZr30LOlUiu1hmQqZ9YidPWMXLJk1tLdoUVg9a', NOW(), pg_read_binary_file('/var/lib/postgresql/def_images/wiktor_profile.png') ),
  ( 'Michał',  '$2b$12$SPTRcKQyxD91xbPmNjNRNuxZcivy3Go7oXW9TfG8JaR60hAhDq3Mq', NOW(), pg_read_binary_file('/var/lib/postgresql/def_images/michal_profile.png') ),
  ( 'Magda',   '$2b$12$SPTRcKQyxD91xbPmNjNRNuxZcivy3Go7oXW9TfG8JaR60hAhDq3Mq', NOW(), pg_read_binary_file('/var/lib/postgresql/def_images/magda_profile.png') ),
  ( 'Krzysiu', '$2b$12$SPTRcKQyxD91xbPmNjNRNuxZcivy3Go7oXW9TfG8JaR60hAhDq3Mq', NOW(), pg_read_binary_file('/var/lib/postgresql/def_images/krzysiu_profile.png') );

INSERT INTO user_groups (vuser_id, vgroup_id, role)
VALUES
  ( 1, 1, 'member' ),
  ( 2, 2, 'member' ),
  ( 2, 1, 'member' ),
  ( 3, 1, 'member' ),
  ( 4, 1, 'member' );

INSERT INTO task (title, content, taskset_id, completed)
VALUES
  ( 'Pętla Kowale', 'Odwiedzić tę niesamowitą pętlę tramwajową', 1, false),
  ( 'Kładka Muchobór', 'Nowa kładka, a jeszcze tam nie byłem', 1, false),
  ( 'Park Tołpy', 'Po prostu park', 1, true ),
  ( 'Antoni Suligowski', '', 2, false ),
  ( 'Jacek Arbaz', '', 2, false ),
  ( 'Bartek Młotek', 'Przynieś ciastka', 2, false ),
  ( 'Tosia Nowak', 'Nocuje!', 2, true ),
  ( 'Masło z solą', 'Lixdark', 3, true ),
  ( 'Łosoś', 'Do ogłupiania miast', 3, true ),
  ( 'Chlebek', 'Po prostu', 3, false);

INSERT INTO task_assign (task_id, user_assign, assign_time)
VALUES
  ( 1, 1, NOW()),
  ( 1, 2, NOW()),
  ( 2, 4, NOW());

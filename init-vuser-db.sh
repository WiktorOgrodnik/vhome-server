#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username postgres --dbname postgres <<-EOSQL
    CREATE USER vuser WITH ENCRYPTED PASSWORD 'secret_pass';
    CREATE DATABASE grouplist;
    GRANT ALL PRIVILEGES ON DATABASE grouplist TO vuser;
EOSQL

psql -v ON_ERROR_STOP=1 --username postgres --dbname grouplist <<-EOSQL
	CREATE EXTENSION POSTGIS
EOSQL


# VHome Server

## How to run in debug mode

You will need cargo and the whole rust toolchain and docker with dokcer compose extension. You may also need a build-essential like package for your distribution if you do not already have it.

1. Create .env file

The server depends on the existence of an .env file. Your .env file should look like this:

```
DATABASE_URL="postgres://postgres:secret_pass@localhost/postgres"
SECRET="YOUR_SECRET"
```

2. Build and run

```bash=
docker compose up --detach
cargo run
```

## How to use deploy script

1. Download and install toolchain for prefered architecture, i.e. `armv7-unknown-linux-gnueabihf`.
2. Install `vhome.service` systemd service on target system.
3. Modify all variables in the `deploy` script.
4. Run deploy script.

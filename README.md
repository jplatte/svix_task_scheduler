# svix take home assignment

## Development setup

Requirements:

- `docker-compose` or `podman-compose`
- `sqlx` CLI (`cargo install sqlx-cli`)

To run a development PostgreSQL server, run:

```sh
podman-compose up -d
```

It is required for running or changing database queries, but not for compilation.

After the database is started for the first time, run

```sh
sqlx db create
sqlx migrate run
```

The latter command has to be re-run whenever a new migration is added.

Whenever a new query macro is added, or an existing one is changed, run

```sh
cargo sqlx prepare
```

to update the `.sqlx` directory to allow builds without the dev database being available.

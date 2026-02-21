# poll-backend

## Local development

1) Start the local Postgres container:
   `./scripts/start-local-postgres.sh`

2) Run the backend:
   `cargo run`

The script writes `poll-backend/.env` with a working `DATABASE_URL`. You can also
copy the template manually:

`cp poll-backend/.env.example poll-backend/.env`

## Environment

- `DATABASE_URL` (required in prod): `postgres://user:pass@host:5432/db`

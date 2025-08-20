# SQLx

## Set up the database for SQLx

```bash
sqlx database setup
```

which also creates the `_sqlx_migrations` table on the database.

Add new migrations with:

```bash
sqlx migrate add <name>
```

Run migrations with:

```bash
sqlx migrate run
```
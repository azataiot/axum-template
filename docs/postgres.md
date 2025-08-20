# PostgreSQL

## Docker Compose

```yaml
services:
  postgres:
    image: postgres:17-alpine
    container_name: postgres
    restart: always
    env_file:
      - .env
    ports:
      - 5432:5432
    volumes:
      - pgdata:/var/lib/postgresql/data
    healthcheck:
      test: [ "CMD-SHELL", "pg_isready -U postgres" ]
      interval: 1h
      timeout: 5s
      retries: 5
    networks:
      - postgres

volumes:
  pgdata:

networks:
  postgres:
    driver: bridge
    name: postgres


```

```ini
POSTGRES_USER=postgres
POSTGRES_PASSWORD=pY0***********
```

## Setting up PostgreSQL role and database

```bash
docker exec -it postgres psql -U postgres
```

```sql
-- 1) create a login role for the app
CREATE ROLE axum_template WITH LOGIN PASSWORD 'REPLACE_WITH_STRONG_PASSWORD';

-- 2) create a dedicated database owned by that role
CREATE DATABASE axum_template OWNER axum_template;

-- 3) connect to it
\c axum_template

-- 4) tighten schema privileges & grant only to your app role
REVOKE CREATE ON SCHEMA public FROM PUBLIC;
GRANT USAGE, CREATE ON SCHEMA public TO axum_template;

-- 5) (optional) enable pgcrypto once, as superuser (needed if you use gen_random_uuid())
-- run this while still connected as superuser (postgres) to the *codhers* DB:
CREATE EXTENSION IF NOT EXISTS pgcrypto;
```
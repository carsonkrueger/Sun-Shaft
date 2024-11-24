Create and setup the .env with the appropriate variables:

```
DATABASE_URL=""
JWT_SECRET=""

ENVIRONMENT="development"
DB_DOMAIN="localhost"
DB_NAME="sun_shaft"
DB_USER="sun_shaft"
DB_PORT=5433
DB_PASSWORD="1234"

BACK_END_DOMAIN="0.0.0.0"
BACK_END_PORT=3000
```

Ensure Postgres is installed on your machine.
Then run `db_init.sh` to create the database locally in the project.
Rerunning this script will delete and recreate the database.
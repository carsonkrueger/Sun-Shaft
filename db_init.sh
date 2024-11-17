# Get .env vars
if [ -f .env ]; then
  export $(cat .env | xargs)
fi

# Check if variables are set
if [[ -z "$DB_NAME" || -z "$DB_USER" || -z "$DB_PASSWORD" || -z "$DB_PORT" ]]; then
  echo "Error: DB_NAME, DB_USER, DB_PORT or DB_PASSWORD is not set in the .env file."
  exit 1
fi

path="/var/lib/postgres/$DB_NAME"
socket_path="$path/socket"

# Stop & delete server
sudo -u postgres pg_ctl -D $path stop
sudo rm -rf $path

# Rebuild server
sudo -u postgres mkdir -p $path
sudo -u postgres initdb -D $path
sudo -u postgres mkdir -p $socket_path
sudo -u postgres pg_ctl -D $path -o "-p $DB_PORT -k $socket_path" start

# Create the database and user
sudo -u postgres psql -p $DB_PORT -h $socket_path -v ON_ERROR_STOP=1 <<-EOSQL
  CREATE USER $DB_USER WITH PASSWORD '$DB_PASSWORD';
  CREATE DATABASE $DB_NAME OWNER $DB_USER;
EOSQL

echo "Database '$DB_NAME':$DB_PORT created with user '$DB_USER'."

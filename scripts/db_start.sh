if [ -f .env ]; then
  export $(cat .env | xargs)
fi

if [[ -z "$DB_NAME" || -z "$DB_PORT" ]]; then
  echo "Error: DB_NAME is not set in the .env file."
  exit 1
fi

path="/var/lib/postgres/$DB_NAME"
socket_path="$path/socket"

sudo -u postgres pg_ctl -D $path -o "-p $DB_PORT -k $socket_path" start

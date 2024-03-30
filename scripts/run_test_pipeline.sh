#!/bin/bash

# move to project directory
SCRIPTPATH="$( cd "$(dirname "$0")"; pwd -P)"
cd "$SCRIPTPATH"
cd ..

# start up docker and wait until it postgres accepts connections
docker-compose up -d
until pg_isready -h localhost -p 5432 -U username
do
  echo "Waiting for postgres"
  sleep 2;
done

cargo build
cargo test

# run server in background
cargo run config.yml &
SERVER_PID=$!
sleep 5

export DYLD_LIBRARY_PATH=/usr/local/Cellar/postgresql@14/14.11_1/lib/postgresql@14/:/usr/local/bin/psql

diesel migration run

cd ./scripts

# create new user
curl --location 'http://localhost:8000/v1/user/create' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "matthew",
    "email": "test@gmail.com",
    "password": "test"
}'

#login to get a fresh token
echo $(curl --location --request GET 'http://localhost:8000/v1/auth/login' \
     --header 'Content-Type: application/json' \
     --data-raw '{
       "username": "matthew",
       "password": "test"
     }') > ./fresh_token.json

TOKEN=$(jq '.token' fresh_token.json)
jq '.auth.apikey.value = '"$TOKEN"'' todo.postman_collection.json > test_newman.json

newman run test_newman.json

rm ./test_newman.json
rm ./fresh_token.json

kill "$SERVER_PID"

cd ..
docker-compose down

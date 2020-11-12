#!/bin/bash

DB_NAME="gumshoe_database"

result=$( docker images -q $DB_NAME )
if [[ -n "$result" ]]; then
    echo "Docker image tagged as $DB_NAME does not exist. Building..."
    docker build -f Database.dockerfile -t $DB_NAME .
else
    echo "Docker image tagged as $DB_NAME exists. Moving forward..."
fi

# Start the database container
docker run $DB_NAME &


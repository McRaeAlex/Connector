#!/bin/bash

DB_NAME="gumshoe_database"

# Check if the image exists and if not build it
image=$( docker images -q $DB_NAME )
if [[ -n "$image" ]]; then
    echo "Docker image tagged as $DB_NAME exists."
else
    echo "Docker image tagged as $DB_NAME does not exist. Building..."
    ./db-build.sh $DB_NAME
fi

# Start the database container
docker run -p 5432:5432 $DB_NAME &> logs/db.log &
echo "Started the container"

export DATABASE_URL="postgresql://dev:dev@localhost:5432/gumshoe"
cargo run | tee logs/server.log


#!/bin/bash

DB_NAME=${1:-"gumshoe_database"} # first arg ($1) or the default value
docker build -f db/Database.dockerfile -t $DB_NAME ./db

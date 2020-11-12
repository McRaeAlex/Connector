#!/bin/bash

docker build -f db/Database.dockerfile -t $DB_NAME ./db
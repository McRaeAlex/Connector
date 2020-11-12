#!/bin/bash

docker build -f Database.dockerfile -t gumshoe_database

docker start gumshoe_database &

# do our table stuff
# migrate the tables up and stuff

cargo run
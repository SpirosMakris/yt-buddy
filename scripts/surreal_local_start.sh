#!/bin/sh
cd "$(dirname "$0")" || exit # change cwd to this script's directory

DB_DATA_DIR=$(readlink -f ../db)
echo $DB_DATA_DIR

DEV_FLAGS="--rm -v $DB_DATA_DIR:/db_data" #  start --log trace --user root --pass root"
SURREAL_IMAGE="--pull always surrealdb/surrealdb:latest"
SURREAL_FLAGS="start --log trace --user root --pass root file:/db_data/my_db.db"

echo "Starting local surrealdb .."
FLAGS="$DEV_FLAGS -p 8000:8000 $SURREAL_IMAGE $SURREAL_FLAGS"

docker run $FLAGS || exit

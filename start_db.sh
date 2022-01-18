#! /bin/bash

docker run --name db -p 0.0.0.0:5432:5432 -e POSTGRES_PASSWORD=password -d postgres

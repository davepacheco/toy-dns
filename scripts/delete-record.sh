#!/bin/bash

if [[ $# -ne 1 ]]; then
    echo "usage: delete-record <name>"
    exit 1
fi

curl -X PUT localhost:5353/delete-records -d "[{\"name\": \"$1\"}]"; echo;

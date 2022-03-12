#!/bin/bash

if [[ $# -ne 5 ]]; then
    echo "usage: add-aaaa <name> <prio> <weight> <port> <target>"
    exit 1
fi

curl -X PUT localhost:5353/set-records \
    -d "[[{\"name\": \"$1\"}, {\"SRV\": [$2, $3, $4, \"$5\"]}]]"

echo;

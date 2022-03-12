#!/bin/bash

if [[ $# -ne 2 ]]; then
    echo "usage: add-aaaa <name> <ipv6>"
    exit 1
fi

curl -X PUT localhost:5353/set-records -d "[[{\"name\": \"$1\"}, {\"AAAA\": \"$2\"}]]"; echo;

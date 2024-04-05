#!/bin/bash

metadata=$(cargo --offline metadata --format-version 1 --no-deps --manifest-path proxmox-api/Cargo.toml)

if [ -z "$metadata" ]; then
    echo "Metadata was empty?" >&2
    exit 1
fi

output=$(echo "$metadata" | jq --raw-output ".packages[] | select(.name == \"proxmox-api\") | .metadata.proxmox.\"$1\"")

if [[ -z "$output" || "$output" = "null" ]]; then
    echo "Could not find requested metadata!" >&2
    exit 1
fi

echo "$output"
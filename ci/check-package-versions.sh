#!/bin/bash

while true; do
    pkg=$1
    shift

    if [ "$pkg" = "" ]; then
        break;
    fi

    name=$(echo $pkg | cut -d '=' -f1)
    version=$(echo $pkg | cut -d '=' -f2)

    latest=$(apt list -qqq $name 2>/dev/null | cut -d ' ' -f2)

    echo "Validating that $name = $latest ($version)"
    if [ ! "$version" = "$latest" ]; then
        echo "::warning::Version of package $name does not match latest available: $latest"
    fi
done
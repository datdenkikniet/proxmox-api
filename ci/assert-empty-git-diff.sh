#!/bin/sh

diff=$(git --no-pager diff)
lines=$(echo -n "$diff" | wc -l)

if [ ! "$lines" -eq "0" ]; then
    echo "Git diff is not empty after running generator! ($lines lines)"
    echo ""
    git --no-pager diff
    exit 1
fi
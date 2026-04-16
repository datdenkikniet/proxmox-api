#!/bin/bash
set -e

wget --quiet https://pve.proxmox.com/pve-docs/api-viewer/apidoc.js

sed '0,/const apiSchema = \[/s//[/' apidoc.js | sed -n '/^\[$/,/^\]$/p' > PVE-schema.json
rm apidoc.js

xz --compress --force PVE-schema.json 1> /dev/null

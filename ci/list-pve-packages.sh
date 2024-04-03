#!/bin/bash

list=$(apt list --installed | grep -E "(^pve-.*)|(proxmox-ve)|(^libpve-.*)" | grep -vE "kernel|headers|firmware|xtermjs")

while IFS= read -r line; do
    name=$(echo $line | cut -d '/' -f1)
    version=$(echo $line | cut -d ' ' -f2)
    packages="$packages $name=$version"
done <<< "$list"

echo $packages

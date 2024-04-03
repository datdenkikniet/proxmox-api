#!/bin/bash

distro=${1:-bookworm}

install() {
    apt update
    apt install -y $@
}

install wget

inst="/etc/apt/sources.list.d/pve-install-repo.list"
mkdir -p $(dirname $inst)
echo "deb http://download.proxmox.com/debian/pve $distro pve-no-subscription" > $inst
wget https://enterprise.proxmox.com/debian/proxmox-release-$distro.gpg -O /etc/apt/trusted.gpg.d/proxmox-release-$distro.gpg

install proxmox-ve || 1

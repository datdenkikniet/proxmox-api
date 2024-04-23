#!/bin/bash

distro=${1:?}

install() {
    apt-get update
    apt-get install -y $@
}

install wget

inst="/etc/apt/sources.list.d/pve-install-repo.list"
mkdir -p $(dirname $inst)
echo "deb http://download.proxmox.com/debian/pve $distro pve-no-subscription" > $inst
wget https://enterprise.proxmox.com/debian/proxmox-release-$distro.gpg -O /etc/apt/trusted.gpg.d/proxmox-release-$distro.gpg

apt-get update
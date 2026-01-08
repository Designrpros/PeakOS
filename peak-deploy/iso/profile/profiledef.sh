#!/usr/bin/env bash
# shellcheck disable=SC2034

iso_name="peakos"
iso_label="PEAKOS_$(date +%Y%m)"
iso_publisher="PeakOS <https://peakos.dev>"
iso_application="PeakOS Live Environment"
iso_version="$(date +%Y.%m.%d)"
install_dir="peakos"
buildmodes=('iso')
bootmodes=('bios.syslinux' 'uefi.grub')
arch="x86_64"
pacman_conf="pacman.conf"
file_permissions=(
  ["/usr/bin/peak-desktop"]="0:0:755"
  ["/usr/bin/peak-intelligence-x86_64-unknown-linux-gnu"]="0:0:755"
)

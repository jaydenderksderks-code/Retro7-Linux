#!/usr/bin/env bash
# Preserve Archiso's current releng defaults, including boot modes and permissions.
source "${ARCHISO_RELENG_DIR:-/usr/share/archiso/configs/releng}/profiledef.sh"

iso_name="retro7"
iso_label="RETRO7_$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y%m)"
iso_publisher="Retro7 Linux"
iso_application="Retro7 Linux Live System"
iso_version="${RETRO7_VERSION:-$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y.%m.%d)}"
install_dir="retro7"

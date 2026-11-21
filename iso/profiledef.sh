#!/usr/bin/env bash
# Retro7 metadata layered onto the Archiso releng profile.
# Boot modes and image layout remain inherited from the installed Archiso profile.

iso_name="retro7"
iso_label="RETRO7_$(date --date="${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y%m)"
iso_publisher="Retro7 Linux"
iso_application="Retro7 Linux Live System"
iso_version="${RETRO7_VERSION:-$(date --date="${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y.%m.%d)}"
install_dir="retro7"

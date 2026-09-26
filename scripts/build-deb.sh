#!/bin/bash

build_variant() {
    local variant=$1
    local output_dir="target/debian"
    
    echo "Building ${variant} package..."
    if [ "$variant" = "default" ]; then
        cargo deb
    else
        cargo deb --variant=$variant
    fi
    
}

default_path=$(build_variant "default")
lite_path=$(build_variant "lite")
# qsvdp stays on the portable x86-64 baseline (no x86-64-v3) - see publish-target.yml
datapusher_plus_path=$(unset RUSTFLAGS; build_variant "datapusher_plus")

echo "DEB_PATHS=${default_path} ${lite_path} ${datapusher_plus_path}"
#!/bin/bash

cargo build --release
cp target/release/spin-static-site static-site
tar -czvf static-site.tar.gz static-site

cat <<EOT > "static-site.json"
{
    "name": "static-site",
    "description": "A plugin to deploy static sites to fermyon cloud",
    "homepage": "www.example.com",
    "version": "0.1.0",
    "spinCompatibility": ">=1.4",
    "license": "Apache-2.0",
    "packages": [
        {
            "os": "macos",
            "arch": "aarch64",
            "url": "file:$(pwd)/static-site.tar.gz",
            "sha256": "$(sha256sum static-site.tar.gz | awk '{print $1;}')"
        }
    ]
}
EOT
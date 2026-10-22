#!/bin/sh
# Builds build/<name>.hammerplugin: manifest.toml and the module, with settings.toml, locales/, and
# resources/ when the plugin has them. Needs rustup's wasm32-unknown-unknown target, and zip.
set -e
cd "$(dirname "$0")"
name=$(sed -n 's/^name = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
cargo build --release --target wasm32-unknown-unknown
rm -rf build/package
mkdir -p build/package
cp manifest.toml build/package/
if [ -f settings.toml ]; then cp settings.toml build/package/; fi
for dir in locales resources; do
	if [ -d "$dir" ]; then cp -r "$dir" build/package/; fi
done
# Cargo names the module with underscores for the crate name's hyphens.
cp "target/wasm32-unknown-unknown/release/$(printf %s "$name" | tr - _).wasm" build/package/plugin.wasm
rm -f "build/$name.hammerplugin"
(cd build/package && zip -q -X -r "../$name.hammerplugin" .)
echo "Built build/$name.hammerplugin"

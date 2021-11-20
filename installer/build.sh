if ! test -f "Cargo.toml"; then
    echo "ERROR: Do not run this inside the installer folder!"
    exit 1
fi
rm -rvf installer/builds/.build_cache
# Build binary
cargo build --release
# Prep
rm -rf installer/payloads/default
mkdir -p installer/payloads/default/bin
cp LICENSE installer/payloads/default/LICENSE
cp ./target/release/pak installer/payloads/default/bin/pak
chmod +x installer/payloads/default/bin/pak
cp LICENSE installer/LICENSE
cd installer
# Build installer
../target/release/pak build
mv builds/out.pkg builds/pak_x86_64.pkg
echo "Built package"

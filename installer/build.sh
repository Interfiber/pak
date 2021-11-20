if ! test -f "Cargo.toml"; then
    echo "ERROR: Do not run this inside the installer folder!"
    exit 1
fi
if [ "$CODESIGN_ID" == "" ]; then
    echo "ERROR: Could not find codesign id in CODESIGN_ID"
    exit 1
fi
if [ "$INSTALLER_ID" == "" ]; then
    echo "ERROR: Could not find installer id in INSTALLER_ID"
    exit 1
fi
rm -rvf installer/builds/.build_cache
# Build binary
cargo build --release
# Prep
rm -rf installer/payloads/default
mkdir -p installer/payloads/default/bin
cp LICENSE installer/payloads/default/LICENSE
cp ./target/release/pak installer/payloads/default/bin/pakcli
chmod +x installer/payloads/default/bin/pakcli
cp LICENSE installer/LICENSE
cd installer
# Codesign
codesign --deep --force --verbose --sign "$CODESIGN_ID" --options=runtime payloads/default/bin/pakcli
# Build installer
../target/release/pakcli build
mv builds/out.pkg builds/pak_x86_64.pkg
echo "Built package"
echo "Press enter to sign package"
read
productsign --sign "$INSTALLER_ID" builds/*.pkg installer.pkg
xcrun notarytool submit --keychain-profile "xcode" --wait installer.pkg
xcrun stapler staple installer.pkg
rm -rf .build_cache
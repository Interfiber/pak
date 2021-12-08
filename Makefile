build_pakage_release:
	# Build Files
	cargo build --release --verbose
	# Prep build folder
	mkdir -p pakinstaller/payloads/default/bin
	mkdir -p pakinstaller/payloads/pakr/bin
	cp -rvf examples pakinstaller/payloads/default/examples
	cp LICENSE pakinstaller/LICENSE
	cp LICENSE pakinstaller/payloads/default/LICENSE
	cp LICENSE pakinstaller/payloads/pakr/LICENSE
	# Copy build files
	cp target/release/pakr pakinstaller/payloads/pakr/bin/pakr
	chmod +x pakinstaller/payloads/pakr/bin/pakr
	cp target/release/pakcli pakinstaller/payloads/default/bin/pakcli
	chmod +x pakinstaller/payloads/default/bin/pakcli
	# Build package
	cd pakinstaller && ../target/release/pakcli build
	# NOTE: Codesign manually
help:
	# Commands
	# build_pakage_release    Builds a release package(CPU intensive task)
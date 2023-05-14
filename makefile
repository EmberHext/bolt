.PHONY: build run setup all api watch build-yew build-tauri watch-yew watch-tauri web clean-yew clean-tauri clean cli build-cli

all: build

# Install required build tools and dependencies
setup:
	cargo install tauri-cli
	cargo install trunk
	rustup target add wasm32-unknown-unknown

# Install Bolt CLI
install-cli:
	cd bolt_cli && cargo install --path .

# Build Bolt Desktop App in release mode
build: build-yew build-tauri
	cp -r ./bolt_tauri/target/release/bundle ./target

# Build Bolt CLI in release mode
build-cli: build-yew
	cd bolt_cli && cargo build --release

# Run Bolt Desktop App in debug mode
run: build-yew watch-tauri

# Run Bolt CLI in debug mode
run-cli: build-yew
	cd bolt_cli && BOLT_DEV=1 cargo run

# Run Bolt Core in headless mode
run-headless: build-yew
	cd bolt_cli && BOLT_DEV=1 cargo run -- --headless

build-yew:
	cd bolt_yew && trunk build -d ../bolt_tauri/dist --filehash false
	cd bolt_yew && cp ./script.js ../bolt_tauri/dist
	mkdir ./bolt_tauri/dist/icon/
	cp -r ./icon/* ./bolt_tauri/dist/icon/ 

build-tauri:
	cd bolt_tauri && cargo tauri build

watch-tauri:
	cargo tauri dev

bump-version:
	ls

publish-libs:
	cd bolt_core/common && cargo publish
	cd bolt_core/http && cargo publish
	cd bolt_core/ws && cargo publish
	cd bolt_core/servers && cargo publish
	cd bolt_core/tcp && cargo publish
	cd bolt_core/udp && cargo publish
	cd bolt_core/core && cargo publish

publish-cli: publish-libs
	cd bolt_cli && cargo publish

# Clean temporary build files
clean: clean-yew clean-tauri clean-cli

clean-yew:
	cd bolt_yew && cargo clean

clean-tauri:
	cd bolt_tauri && cargo clean

clean-cli:
	cd bolt_cli && cargo clean

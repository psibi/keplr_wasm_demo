# List all recipes
default:
	just --list --unsorted

# Build wasm file
build-wasm:
	cargo build --target wasm32-unknown-unknown --release

# Install
install:
	yarn

# Preview
preview:
	yarn run dev

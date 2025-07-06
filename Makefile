check:
	cargo check

lint:
	cargo clippy -- -D warnings

format:
	cargo fmt -- --check

test:
	cargo test -- --nocapture

build:
	cargo build

run:
	NODE_IP=127.0.0.1 NODE_PORT=9999 cargo run

graph:
	 docker compose down --volumes --remove-orphans
	 docker compose up --build --force-recreate --remove-orphans
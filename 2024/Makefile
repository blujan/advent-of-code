.PHONY: test generate run format clippy

generate:
	cargo generate --path ./template --name day-${day}

test:
	cargo test --package day-${day}

run:
	cargo run day-${day}

format:
	cargo fmt --package day-${day}

clippy:
	cargo clippy --package day-${day}

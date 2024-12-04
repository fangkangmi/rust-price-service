.PHONY: build test run list_packages

PROJECT_NAME := your_project_name

build: list_packages
	cargo build -p $(PROJECT_NAME)

test: list_packages
	cargo test -p $(PROJECT_NAME)

run: list_packages
	cargo run -p $(PROJECT_NAME)
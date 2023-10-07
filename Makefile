.PHONY: fetch lint build local-image-embed start-embed

# Fetches new dependencies from cargo registries
fetch:
	cargo fetch --locked 

# Lints rust code via fmt and clippy
lint:
	cargo fmt
	cargo clippy --fix --allow-dirty

# Checks formatting, tests code, and builds binaries
build:
	cargo fmt -- --check
	cargo test --locked
	cargo build --locked --release --all

local-image-embed:
	docker buildx build -f Dockerfile.embed --load -t ircc-ai-embed .

local-image: local-image-embed

start-embed:
	CONTENT_PATH_HOST=$(CONTENT_PATH_HOST) docker-compose -f docker-compose.yml --profile manual up embed --build

.PHONY: fetch lint build local-image-embed start-embed local-image-oracle start-oracle

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
	docker buildx build -f embed.Dockerfile --load -t ircc-ai-embed .

local-image-oracle:
	docker buildx build -f oracle.Dockerfile --load -t ircc-ai-oracle .

local-image: local-image-embed local-image-oracle

start-embed:
	CONTENT_PATH_HOST=$(CONTENT_PATH_HOST) docker-compose -f docker-compose.yml --profile manual up embed --build

start-oracle:
	CONTENT_PATH_HOST=$(CONTENT_PATH_HOST) docker-compose -f docker-compose.yml up oracle --build
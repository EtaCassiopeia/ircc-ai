.PHONY: fetch lint build local-image-embed start-embed local-image-oracle start-oracle local-image-bot start-bot local-image

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
	cargo clean
	docker buildx build --platform linux/amd64 -f Dockerfile.embed.x86_64 --load -t ircc-ai-embed .

local-image-oracle:
	cargo clean
	docker buildx build --platform linux/amd64 -f Dockerfile.oracle.x86_64 --load -t ircc-ai-oracle .

local-image-bot:
# Clean up the build catch to avoid copying unnecessary files into the image.
# This can be done by adding the target folder into .dockerignore file but Docker engine will ignore the folder even during the build process.
	cargo clean
	docker buildx build --platform linux/amd64 -f Dockerfile.bot.x86_64 -t ircc-ai-bot .

local-image: local-image-embed local-image-oracle local-image-bot

start-embed:
	CONTENT_PATH_HOST=$(CONTENT_PATH_HOST) docker-compose -f docker-compose.yml --profile manual-embed up embed --build

start-oracle:
	CONTENT_PATH_HOST=$(CONTENT_PATH_HOST) docker-compose -f docker-compose.yml up oracle --build

start-bot:
	docker-compose -f docker-compose.yml --profile manual-bot up bot --build
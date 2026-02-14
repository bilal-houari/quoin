# Quoin Development Justfile

uid := `id -u`
gid := `id -g`

# Build the web interface using Docker
build-web:
    docker build -t quoin-web-builder -f Dockerfile.web .
    docker run --rm -v {{invocation_directory()}}:/app -e USER_ID={{uid}} -e GROUP_ID={{gid}} quoin-web-builder

# Run the backend (builds if necessary)
# Assumes web/dist already exists from build-web
run:
    cargo run -- server --port 3232 --allow-external

# Run the backend with watch mode (auto-reloads on Rust OR Frontend changes)
watch:
    cargo watch -w src -w web/dist -x 'run -- server --port 3232 --allow-external'

# Build everything (convenience)
build: build-web
    cargo build

# Build the production Docker image
docker-image port="3232":
    docker build -t quoin --build-arg PORT={{port}} .

# Run the production image as a server (ephemeral, local-only by default)
docker-run:
    docker run -it --init --rm -p 3232:3232 quoin server --allow-external

# Run the production image as a CLI tool (mounts current dir to /data)
docker-cli *args:
    docker run -it --init --rm -v {{invocation_directory()}}:/data quoin {{args}}

# Full iterative flow: build web then run backend
dev: build-web run

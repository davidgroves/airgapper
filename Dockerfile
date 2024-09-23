# Build Stage
FROM rust:1.80.1-alpine AS build

WORKDIR /usr/local/src/

# Install tools needed to build against MUSL libc.
RUN USER=root apk add --no-cache musl-dev

# Install cross compiler targets.
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add armv7-unknown-linux-musleabi

# Make this project.
RUN cargo new airgapper

# Install crosscompiler tools in project.
RUN cargo install -f cross

# Copy things needed to build into container.
WORKDIR /usr/local/src/airgapper
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .cargo ./.cargo

# Build the binaries.
RUN cargo build --release --target=x86_64-unknown-linux-musl --bin airgapper-x86_64
RUN cargo build --release --target=armv7-unknown-linux-musleabi --bin airgapper-armv7

# Output the binaries to the host machine.
FROM scratch AS artifact
COPY --from=build /usr/local/src/airgapper/target/x86_64-unknown-linux-musl/release/airgapper-x86_64 bin/airgapper-x86_64-static
COPY --from=build /usr/local/src/airgapper/target/armv7-unknown-linux-musleabi/release/airgapper-armv7 bin/airgapper-armv7-static

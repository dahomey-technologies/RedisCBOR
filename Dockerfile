# Setup
FROM rust as setup
RUN apt-get update
RUN apt-get install -y clang
ENV LIBCLANG_PATH=/usr/lib/x86_64-linux-gnu
RUN cargo install cargo-chef 
WORKDIR /build

# Prepare
FROM setup as prepare
COPY src ./src
COPY Cargo.toml .
RUN cargo chef prepare --recipe-path recipe.json

# Cache
FROM setup as cook
COPY --from=prepare /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build
FROM cook as build
COPY . .
RUN cargo build --release

# Runtime
FROM redis
WORKDIR /data
COPY --from=build /build/target/release/librecbor.so /
CMD ["redis-server", "--loadmodule", "/librecbor.so"]
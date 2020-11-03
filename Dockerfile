FROM rustlang/rust:nightly-buster AS builder
# ref. https://github.com/rust-lang/docker-rust-nightly

WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./

# minimum compilable main.rs
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/hello_rust*

COPY ./src ./src
RUN cargo build --release

FROM debian:10.6-slim

COPY --from=builder /app/target/release/hello-rust /app/target/release/hello-rust
CMD ROCKET_PORT=$PORT /app/target/release/hello-rust

FROM rustlang/rust:nightly-buster
# ref. https://github.com/rust-lang/docker-rust-nightly

WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./

# minimum compilable main.rs
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release

COPY . .
RUN cargo build --release

CMD ROCKET_PORT=$PORT /app/target/release/hello-rust

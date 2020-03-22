FROM rustlang/rust:nightly-buster
# ref. https://github.com/rust-lang/docker-rust-nightly

WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./
# lib.rs or main.rsがないとerrorになる
RUN mkdir src && touch src/lib.rs
RUN cargo build --release

COPY . .

CMD ['cargo', 'run', '--release']

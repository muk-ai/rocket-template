FROM rust:1.56-buster AS builder

WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./

# minimum compilable main.rs
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/rocket_template_app*

COPY ./src ./src
COPY ./migrations ./migrations
RUN cargo build --release

FROM debian:10.6-slim

RUN apt-get update -qq \
  && apt-get install -y libpq-dev \
  && apt-get install -y --no-install-recommends ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/rocket-template-app /usr/local/bin/
COPY ./Rocket.toml /app
COPY ./public /app/public

CMD ROCKET_PORT=$PORT /usr/local/bin/rocket-template-app

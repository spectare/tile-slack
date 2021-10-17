# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:1.54-buster as cargo-build

RUN apt-get update

WORKDIR /usr/src/forms

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

COPY . .

RUN cargo build --release

# RUN cargo install --path .

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM debian:bullseye-slim

RUN apt-get update && rm -rf /var/lib/apt/lists/*

RUN addgroup --system -gid 1000 runtme

RUN adduser --system --disabled-login --shell /bin/sh -uid 1001 --ingroup runtme runtme

COPY --from=cargo-build /usr/src/forms/target/release/tile-slack /usr/local/bin/tile-slack

RUN chown runtme:runtme /usr/local/bin/tile-slack

USER runtme

CMD ["tile-slack"]

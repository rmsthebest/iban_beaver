FROM rust as builder

RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /opt
# create a new empty shell project
RUN USER=root cargo new --bin iban_beaver
WORKDIR /opt/iban_beaver
# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# this build step will cache your dependencies
RUN cargo build --release
RUN rm ./src/*.rs
RUN rm ./target/release/deps/iban_beaver*

# copy your source tree
COPY ./src ./src
COPY ./resources ./resources
COPY ./.env ./.env 
ENV IBAN_BEAVER_RESOURCES="./resources"
RUN cargo install diesel_cli --no-default-features --features "sqlite-bundled"
RUN diesel setup
RUN diesel migration run
RUN cargo build --release

FROM debian:buster-slim as scratch
#FROM rust as scratch
RUN apt-get update && apt install -y libcurl4 sqlite3 libsqlite3-dev
WORKDIR /opt/iban_beaver
# copy the build artifact from the build stage
COPY --from=builder /opt/iban_beaver/target/release/iban_beaver .
COPY ./resources ./resources
COPY ./images ./images
COPY ./migrations ./migrations

EXPOSE 3030
# set the startup command to run your binary
CMD ["./iban_beaver"]
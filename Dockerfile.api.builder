FROM rust:1.58-slim-buster as builder

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq-dev pkg-config libssl-dev
RUN mkdir -p /code
WORKDIR /code
RUN USER=root cargo new --bin api
RUN USER=root cargo new --lib repo_pg
RUN USER=root cargo new --bin cli
RUN USER=root cargo new --lib extra
RUN USER=root cargo new --lib macros
RUN USER=root cargo new --lib axum-derive
RUN USER=root cargo new --lib tests
COPY ./Cargo.toml /code/Cargo.toml
COPY ./api/Cargo.toml /code/api/Cargo.toml
COPY ./repo_pg/Cargo.toml /code/repo_pg/Cargo.toml
COPY ./cli/Cargo.toml /code/cli/Cargo.toml
WORKDIR /code/repo_pg
RUN cargo build --release
WORKDIR /code/api
RUN cargo build --release
WORKDIR /code/cli
RUN cargo build --release
WORKDIR /code
RUN rm api/src/*.rs
RUN rm repo_pg/src/*.rs
RUN rm cli/src/*.rs

ADD ./api ./api
ADD ./repo_pg ./repo_pg
ADD ./cli ./cli

WORKDIR /code/api
RUN cargo build --release
WORKDIR /code/cli
RUN cargo build --release


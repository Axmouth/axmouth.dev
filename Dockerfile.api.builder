FROM rust:1.50-slim-buster as builder

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq-dev pkg-config libssl-dev
RUN mkdir -p /code
WORKDIR /code
RUN USER=root cargo new --bin backend_api
RUN USER=root cargo new --bin backend_repo_pg
RUN USER=root cargo new --bin backend_cli
COPY ./Cargo.toml /code/Cargo.toml
COPY ./backend_api/Cargo.toml /code/backend_api/Cargo.toml
COPY ./backend_repo_pg/Cargo.toml /code/backend_repo_pg/Cargo.toml
COPY ./backend_cli/Cargo.toml /code/backend_cli/Cargo.toml
WORKDIR /code/backend_repo_pg
RUN cargo build --release
WORKDIR /code/backend_api
RUN cargo build --release
WORKDIR /code/backend_cli
RUN cargo build --release
WORKDIR /code
RUN rm backend_api/src/*.rs
RUN rm backend_repo_pg/src/*.rs
RUN rm backend_cli/src/*.rs

ADD ./backend_api ./backend_api
ADD ./backend_repo_pg ./backend_repo_pg
ADD ./backend_cli ./backend_cli

RUN rm ./target/release/deps/backend_api*
WORKDIR /code/backend_api
RUN cargo build --release
WORKDIR /code/backend_cli
RUN cargo build --release


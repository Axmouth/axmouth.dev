FROM rust:1.46-slim-buster as builder

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

RUN rm ./target/release/deps/api*
WORKDIR /code/api
RUN cargo build --release
WORKDIR /code/cli
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq-dev \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 39051

ENV TZ=Etc/UTC \
    APP_USER=appuser
ENV APP=/var/lib/axmouth/axmouth.dev/backend

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}
RUN mkdir -p ${APP}
COPY --from=builder /code/target/release/api ${APP}
COPY --from=builder /code/target/release/cli ${APP}
RUN touch ${APP}/.env

RUN chown -R $APP_USER:$APP_USER ${APP}
RUN chmod +x ${APP}/api
RUN chmod +x ${APP}/cli


COPY initialize-static-file-volume-permissions.sh /entrypoint.sh
ENTRYPOINT ["/bin/sh", "/entrypoint.sh"]

WORKDIR ${APP}

CMD ["/var/lib/axmouth/axmouth.dev/backend/api"]
FROM rust:1.46-slim-buster as builder

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
COPY --from=builder /code/target/release/backend_api ${APP}
COPY --from=builder /code/target/release/backend_cli ${APP}
RUN touch ${APP}/.env

RUN chown -R $APP_USER:$APP_USER ${APP}
RUN chmod +x ${APP}/backend_api
RUN chmod +x ${APP}/backend_cli


COPY initialize-static-file-volume-permissions.sh /entrypoint.sh
ENTRYPOINT ["/bin/sh", "/entrypoint.sh"]

WORKDIR ${APP}

CMD ["/var/lib/axmouth/axmouth.dev/backend/backend_api"]
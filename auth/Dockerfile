FROM rust:1.52 as builder

RUN rustup default nightly
RUN USER=root cargo new --bin authentication_api

WORKDIR /usr/app/

COPY ./Cargo.toml ./Cargo.lock .

RUN mkdir src/ && echo "fn main() {}" > src/main.rs

RUN cargo build --release
COPY . ./
RUN touch src/main.rs && cargo build --release

FROM debian:buster-slim

ARG APP=/usr/src/app

ENV TZ=Etc/UTC \
    APP_USER=appuser
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder --chown=$APP_USER:$APP_USER /usr/app/target/release/auth-example ${APP}/auth-example

USER $APP_USER
WORKDIR ${APP}

ENV ROCKET_ENV production

CMD ["./auth-example"]

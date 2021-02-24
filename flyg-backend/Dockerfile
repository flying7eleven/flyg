# the container used for building the backend
FROM rustlang/rust:nightly-buster-slim as build_environment
USER root
WORKDIR /build
COPY . .
RUN apt update -y && apt install libpq-dev -y
RUN cargo build --release

# the actual container used for running the backend
FROM debian:buster-slim
RUN apt update -y && apt install libpq5 curl -y
COPY --from=build_environment /build/target/release/flyg-backend /usr/bin/flyg
WORKDIR /etc/flyg
CMD ["/usr/bin/flyg", "run"]

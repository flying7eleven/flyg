# the container used for building the backend
FROM rustlang/rust:nightly-alpine as build_environment
USER root
WORKDIR /build
COPY . .
RUN apk update && apk add g++ postgresql-dev
RUN cargo build --release

# the actual container used for running the backend
FROM alpine
COPY --from=build_environment /build/target/release/flyg-backend /usr/bin/flyg
WORKDIR /etc/flyg
CMD ["/usr/bin/flyg", "run"]

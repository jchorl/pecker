FROM rust:1.42 as build
WORKDIR /build
COPY . .
RUN cargo build --release

FROM ubuntu:19.10
RUN apt-get update && apt-get install -y \
    smartmontools
COPY --from=build /build/target/release/pecker /bin/
ENTRYPOINT ["/bin/pecker"]

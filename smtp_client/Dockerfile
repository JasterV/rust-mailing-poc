FROM rust:1.68.0 as builder
ENV NAME=smtp_client

# First build a dummy project with our dependencies to cache them in Docker
WORKDIR /usr/src
RUN cargo new --bin ${NAME}
WORKDIR /usr/src/${NAME}
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm ./src/*.rs
RUN rm ./target/release/deps/${NAME}*

# Now copy the sources and do the real build
COPY ./src/* ./src/
RUN cargo build --release 

# Second stage putting the build result into a debian jessie-slim image
FROM debian:buster-slim
ENV NAME=smtp_client

RUN apt-get update \
  && apt-get install -y ca-certificates tzdata \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/${NAME}/target/release/${NAME} /usr/local/bin/${NAME}
CMD ${NAME}

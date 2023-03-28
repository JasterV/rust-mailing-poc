FROM rust:1.68.0 as builder

ENV NAME=imap_client
ENV LIB=deadpool_imap

# First build a dummy project with our dependencies to cache them in Docker
WORKDIR /usr/src

RUN cargo new --bin ${NAME}
RUN cargo new --lib ${LIB} 

COPY ./${NAME}/Cargo.toml ./${NAME}/Cargo.toml
COPY ./${LIB}/Cargo.toml ./${LIB}/Cargo.toml

WORKDIR /usr/src/${NAME}

RUN cargo build --release

WORKDIR /usr/src/

RUN rm ./${NAME}/src/*.rs
RUN rm ./${LIB}/src/*.rs
RUN rm ./${NAME}/target/release/deps/${NAME}*
RUN rm ./${NAME}/target/release/deps/${LIB}*
RUN rm ./${NAME}/target/release/deps/lib${LIB}*

# Now copy the sources and do the real build
COPY ./${NAME}/src/* ./${NAME}/src/
COPY ./${LIB}/src/* ./${LIB}/src/

WORKDIR /usr/src/${NAME}

RUN cargo build --release

# Second stage putting the build result into a debian jessie-slim image
FROM debian:buster-slim
ENV NAME=imap_client

RUN apt-get update \
  && apt-get install -y ca-certificates tzdata \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/${NAME}/target/release/${NAME} /usr/local/bin/${NAME}
CMD ${NAME}

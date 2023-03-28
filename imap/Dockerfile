FROM rust:1.68.0 as builder

ENV NAME=imap_client
ENV LIB=deadpool_imap

# First build a dummy project with our dependencies to cache them in Docker
WORKDIR /usr/src

RUN cargo new --bin ${NAME}
RUN cargo new --lib ${LIB} 

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

COPY ./${NAME}/Cargo.toml ./${NAME}/Cargo.toml

COPY ./${LIB}/Cargo.toml ./${LIB}/Cargo.toml

RUN cargo build -p ${NAME} --release

RUN rm ./${NAME}/src/*.rs
RUN rm ./${LIB}/src/*.rs
RUN rm ./target/release/deps/${NAME}*
RUN rm ./target/release/deps/${LIB}*
RUN rm ./target/release/deps/lib${LIB}*

# Now copy the sources and do the real build
COPY ./${NAME}/src/* ./${NAME}/src/
COPY ./${LIB}/src/* ./${LIB}/src/

RUN cargo build -p ${NAME} --release

# Second stage putting the build result into a debian jessie-slim image
FROM debian:buster-slim
ENV NAME=imap_client

RUN apt-get update \
  && apt-get install -y ca-certificates tzdata \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/target/release/${NAME} /usr/local/bin/${NAME}
CMD ${NAME}

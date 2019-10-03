FROM rust:1.38 as builder

ARG OPENSSL_VERSION="openssl-1.0.2r"

RUN apt-get update && \
  apt-get install -y --no-install-recommends \
  ca-certificates musl-tools libssl-dev

# openssl
RUN mkdir -p /usr/local/musl/include && \
    ln -s /usr/include/linux /usr/local/musl/include/linux && \
    cd /tmp && \
    curl -LO "https://www.openssl.org/source/$OPENSSL_VERSION.tar.gz" && \
    tar xvzf "$OPENSSL_VERSION.tar.gz" && \
    cd "$OPENSSL_VERSION" && \
    env CC=musl-gcc ./Configure no-shared no-zlib \
      -fPIC \
      --prefix=/usr/local/musl \
      -DOPENSSL_NO_SECURE_MEMORY linux-x86_64 && \
    env C_INCLUDE_PATH=/usr/local/musl/include/ make depend && \
    env C_INCLUDE_PATH=/usr/local/musl/include/ make && \
    make install && \
    rm /usr/local/musl/include/linux && \
    rm -r /tmp/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

ENV OPENSSL_DIR=/usr/local/musl/ \
    OPENSSL_INCLUDE_DIR=/usr/local/musl/include/ \
    DEP_OPENSSL_INCLUDE=/usr/local/musl/include/ \
    OPENSSL_LIB_DIR=/usr/local/musl/lib/ \
    OPENSSL_STATIC=1

RUN make build:release

FROM alpine:3.10

RUN apk add --no-cache ca-certificates

WORKDIR /
COPY --from=builder /app/key.json .
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/ripsaw .

CMD ["/ripsaw"]

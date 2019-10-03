FROM rust:1.38 as builder

RUN apt-get update && \
  apt-get install -y musl-tools

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

RUN make build:release

FROM alpine:3.10

RUN apk add --no-cache ca-certificates

WORKDIR /
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/ripsaw .

CMD ["/ripsaw"]

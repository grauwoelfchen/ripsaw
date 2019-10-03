FROM rust:1.38 as builder

RUN apt-get update && \
  apt-get install -y musl-tools

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

RUN make

FROM alpine:3.10

RUN apk add --no-cache ca-certificates

WORKDIR /
COPY --from=builder /app/ripsaw .

CMD ["/ripsaw"]

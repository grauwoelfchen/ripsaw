FROM rust:1.38 as builder

WORKDIR /app
COPY . .

RUN rustc src/main.rs -o ripsaw

FROM alpine:3.10

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/ripsaw

CMD ["/ripsaw"]

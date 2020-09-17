FROM ekidd/rust-musl-builder as builder

WORKDIR /home/rust/src
COPY --chown=rust:rust . .

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:3.12

WORKDIR /

RUN apk add --no-cache ca-certificates && update-ca-certificates

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

COPY --chown=nobody:nogroup --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/cidr-chef cidr-chef
RUN ["chmod", "a+x", "/cidr-chef"]
EXPOSE 8080
USER nobody
ENTRYPOINT ["/cidr-chef"]

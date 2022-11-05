FROM rust:1.40 as builder

WORKDIR /asprn
COPY responder/ .

RUN cargo install --path .
RUN cargo install dtn7

FROM alpine:latest
# RUN apk add extra-runtime-dependencies 
WORKDIR /usr/local/cargo/bin/
COPY --from=builder /usr/local/cargo/bin/* .

WORKDIR /asprn
COPY asprn/* .

CMD ["run.sh"]

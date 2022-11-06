FROM rust:1.65-slim-buster

WORKDIR /asprn/responder-dir
COPY responder/ .

RUN cargo install --path .
RUN cargo install dtn7

ENV PATH="${PATH}:/asprn"
WORKDIR /asprn

RUN cp /asprn/responder-dir/target/debug/responder .
RUN cp /usr/local/cargo/bin/dtn* .
ADD asprn/run.sh ./run.sh

RUN chmod +x run.sh
CMD ["./run.sh"]

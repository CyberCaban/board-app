FROM alpine:3.20.2

RUN apk add --no-cache curl bash rust cargo libpq-dev

RUN cargo --version

WORKDIR /app


RUN cargo install diesel_cli --version 2.2.3 --no-default-features --features postgres
ENV PATH="/root/.cargo/bin:${PATH}"
RUN diesel --help

COPY . ./
RUN cargo build --release

CMD diesel migration run && cargo run --release
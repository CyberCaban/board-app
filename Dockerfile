ARG NODE_VERSION=20.15.0
FROM node:${NODE_VERSION}-alpine AS node
FROM alpine:3.20.2

RUN apk add --no-cache curl bash rust cargo libpq-dev

COPY --from=node /usr/lib /usr/lib
COPY --from=node /usr/local/lib /usr/local/lib
COPY --from=node /usr/local/include /usr/local/include
COPY --from=node /usr/local/bin /usr/local/bin
RUN node -v

RUN cargo --version

WORKDIR /app
COPY ./www/package.json ./www/package-lock.json ./www/
RUN cd www \
    && npm install \
    && cd ..

COPY . ./
RUN cd www \
    && npm run build \
    && cd ..

RUN cargo install diesel_cli --version 2.2.3 --no-default-features --features postgres
ENV PATH="/root/.cargo/bin:${PATH}"
RUN diesel --help

RUN cargo build --release
RUN cp target/release/web-app /app
RUN rm -rf target

CMD diesel migration run && /app/web-app
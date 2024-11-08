FROM node:20.18.0-alpine AS node
FROM alpine:3.20.2

COPY --from=node /usr/lib /usr/lib
COPY --from=node /usr/local/lib /usr/local/lib
COPY --from=node /usr/local/include /usr/local/include
COPY --from=node /usr/local/bin /usr/local/bin

RUN node -v

WORKDIR /app
COPY ./package.json ./package-lock.json ./
RUN npm install

COPY . ./
RUN npm run build

CMD npm run preview
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
ARG NEXT_PUBLIC_API_BASE_URL
ENV NEXT_PUBLIC_API_BASE_URL=${NEXT_PUBLIC_API_BASE_URL}

COPY . ./

RUN npm run build 
CMD npm run start
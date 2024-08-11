FROM rust:alpine

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY . .
RUN cargo build -r

FROM alpine
RUN mkdir /content

ENV CONTENT_DISPLAY_PATH=https://localhost/images/
ENV SERVER_URL=https://localhost/
ENV CONTENT_PATH=/content/
ENV API_URL=https://localhost/api/

WORKDIR /app

COPY --from=0 /app/target/release/my_collection /

EXPOSE 3000

CMD ["/my_collection"]


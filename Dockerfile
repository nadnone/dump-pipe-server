FROM docker.io/alpine:edge

RUN apk update && apk upgrade
RUN apk add git cargo rustup

RUN mkdir /app
WORKDIR /app/
RUN git clone https://github.com/nadnone/dump-pipe-server.git


WORKDIR /app/dump-pipe-server/
RUN cargo build --release

CMD git pull && /app/dump-pipe-server/target/release/dump-pipe-messages-server

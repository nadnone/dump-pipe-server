FROM docker.io/alpine:edge

RUN apk update && apk upgrade
RUN apk add git cargo rustup

RUN mkdir /app
WORKDIR /app/
RUN git clone https://github.com/nadnone/dump-pipe-server.git

WORKDIR /app/dump-pipe-server/
RUN cargo build --release 

RUN cp /app/dump-pipe-server/target/release/dump-pipe-messages-server /app/server

WORKDIR /app/

CMD cd /app/dump-pipe-server/ git pull && cd /app && ./server 
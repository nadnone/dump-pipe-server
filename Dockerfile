FROM docker.io/alpine:edge

RUN apk update && apk upgrade
RUN apk add git cargo rustup

RUN mkdir /app
WORKDIR /app/

RUN addgroup -S dump-server && adduser -S dump-server -G dump-server
RUN chown -R dump-server:dump-server /app
USER dump-server

RUN git clone https://github.com/nadnone/dump-pipe-server.git

WORKDIR /app/dump-pipe-server/
RUN cargo build --release 

RUN cp /app/dump-pipe-server/target/release/dump-pipe-messages-server /app/server

WORKDIR /app/

CMD cd /app/dump-pipe-server/ git pull && cd /app && ./server & touch $(date +"./logs_%Y-%m-%d.txt") && tail -f $(date +"/app/logs_%Y-%m-%d.txt")
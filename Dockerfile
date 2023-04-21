FROM node:alpine as client-builder

RUN npm install --global pnpm

ADD . /app
WORKDIR /app

RUN pnpm install
RUN pnpm build

FROM rust:bullseye as server-builder

RUN apt-get update
RUN apt-get -y install \
        build-essential \
        libssl-dev \
        libgit2-dev \
        libc6-dev \
        libpq-dev \
        libsqlite3-dev \
        lld

ADD . /app
WORKDIR /app

COPY --from=client-builder /app/build /app/build

ENV RUSTFLAGS="-Z gcc-ld=lld -C target-feature=-crt-static"
RUN cargo build --release

FROM ubuntu:20.04

RUN apt-get update
RUN apt-get -y install libssl1.1 ca-certificates openssl
RUN rm -rf /var/lib/apt/lists/*

RUN mkdir -p /app
WORKDIR /app

COPY --from=server-builder /app/target/release/mayhem /app
COPY --from=server-builder /app/target/release/mayhemctl /app
COPY --from=server-builder /app/target/release/mayhem-migrations /app

ADD docker/startup.sh /app/startup.sh

EXPOSE 4000
EXPOSE 4001

CMD [ "/bin/bash", "/app/startup.sh" ]

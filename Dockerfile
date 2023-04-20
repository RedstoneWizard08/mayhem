FROM rust:alpine as builder

RUN apk add --no-cache \
        musl-dev \
        openssl-dev \
        libgit2-dev \
        libc6-compat \
        libc-dev \
        nodejs-current \
        npm \
        postgresql-dev \
        sqlite-dev \
        lld

RUN npm install --global pnpm

ADD . /app
WORKDIR /app

RUN pnpm install --no-optional
RUN pnpm build

ENV RUSTFLAGS="-Z gcc-ld=lld -C target-feature=-crt-static"
RUN cargo build --release

FROM alpine

RUN apk add --no-cache libc6-compat
RUN mkdir -p /app
WORKDIR /app

COPY --from=builder /app/target/release/mayhem /app
COPY --from=builder /app/target/release/mayhemctl /app
COPY --from=builder /app/target/release/mayhem-migrations /app

ADD docker/startup.sh /app/startup.sh

EXPOSE 4000
EXPOSE 4001

CMD [ "/bin/ash", "/app/startup.sh" ]

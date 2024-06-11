FROM ubuntu:20.04

ARG TARGETARCH

RUN apt-get update
RUN apt-get -y install ca-certificates wget

RUN mkdir -p /app
WORKDIR /app

RUN case "$(uname -m)" in \
        aarch64) REPO="http://ports.ubuntu.com/ubuntu-ports" ;; \
        x86_64) REPO="http://security.ubuntu.com/ubuntu" ;; \
        *) echo "Unknown system!"; exit 1 ;; \
    esac && \
    wget -O libssl.deb \
        "$REPO/pool/main/o/openssl1.0/libssl1.0.0_1.0.2n-1ubuntu5.12_$(dpkg --print-architecture).deb" && \
    dpkg -i libssl.deb && \
    rm libssl.deb

ADD "dist/target/server/${TARGETARCH}/release/server" /app
ADD "dist/target/cli/${TARGETARCH}/release/mayhemctl" /app
ADD "dist/target/migrations/${TARGETARCH}/release/migrations" /app

ADD docker/startup.sh /app/startup.sh

EXPOSE 4000
EXPOSE 4001

CMD [ "/bin/bash", "/app/startup.sh" ]

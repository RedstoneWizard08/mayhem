FROM ubuntu:20.04

ARG TARGETARCH

RUN apt-get update
RUN apt-get -y install ca-certificates

RUN mkdir -p /app
WORKDIR /app

ADD "target/${TARGETARCH}/release/mayhem" /app
ADD "target/${TARGETARCH}/release/mayhemctl" /app
ADD "target/${TARGETARCH}/release/mayhem-migrations" /app

ADD docker/startup.sh /app/startup.sh

EXPOSE 4000
EXPOSE 4001

CMD [ "/bin/bash", "/app/startup.sh" ]

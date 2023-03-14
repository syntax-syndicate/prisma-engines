FROM --platform=linux/amd64 node:18

RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get -y install gdb

ENV PRISMA_QUERY_ENGINE_LIBRARY=/engines/target/x86_64-unknown-linux-gnu/debug/libquery_engine.node

WORKDIR /app

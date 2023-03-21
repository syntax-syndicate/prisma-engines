FROM fedora

RUN dnf -y update
RUN dnf -y groupinstall 'Development Tools'
RUN dnf -y install rr gdb nodejs nodejs-npm rustc cargo
RUN dnf -y install openssl1.1-devel
RUN dnf -y install which g++ python
RUN dnf -y install clang lld

ENV PRISMA_QUERY_ENGINE_LIBRARY=/engines/target/debug/libquery_engine.node
WORKDIR /client

RUN echo "set debuginfod enabled" > /root/.gdbinit

# ENV RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=lld"

CMD [bash]

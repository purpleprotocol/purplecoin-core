FROM rustlang/rust:nightly-alpine

RUN apk update  && \
    apk upgrade  && \
    apk add make \
            g++ \
            m4 \
            cmake \ 
            clang \ 
            clang-dev \
            clang-libs \
            llvm \
            llvm-dev \
            musl \
            musl-dev

WORKDIR /usr/src/purplecoin

COPY . .

ENV LLVM_CONFIG_PATH /usr/bin/llvm‑config

RUN cargo +nightly install --profile release --no-default-features --features "rpc wallet disk miner blake3sum" --path .
RUN rm -rf /usr/src/purplecoin

CMD ["/usr/local/cargo/bin/purplecoin"]
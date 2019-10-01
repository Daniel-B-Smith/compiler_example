FROM ubuntu:18.04

RUN apt-get update && apt-get install -y curl

RUN echo "increment for cache busting: 0"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH /root/.cargo/bin/:$PATH

RUN apt-get update && apt-get install -y build-essential \
    llvm-3.9-dev libclang-3.9-dev clang-3.9

WORKDIR /app

# Hack to cache the dependency builds.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src
RUN echo "fn main() {} // This is a hack" >> src/main.rs
RUN cargo build --release
RUN rm -f src/*.rs

COPY ./src ./src
RUN cargo build --release

RUN cp /app/target/release/compiler_example /usr/local/bin/compiler_example \
 && chmod +x /usr/local/bin/compiler_example

RUN rm -rf /app

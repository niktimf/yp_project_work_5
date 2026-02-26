FROM rust:latest

RUN apt-get update && apt-get install -y \
    valgrind \
    linux-perf \
    && rm -rf /var/lib/apt/lists/*

# nightly для miri и sanitizers
RUN rustup toolchain install nightly && rustup component add miri --toolchain nightly

WORKDIR /app
COPY . .

CMD ["bash"]
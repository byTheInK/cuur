FROM opensuse/tumbleweed:latest

RUN zypper refresh && \
    zypper -n install curl gcc make pkg-config patterns-devel-base-devel_basis

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY . /cuur
RUN chmod +x /cuur/scripts/build

WORKDIR /cuur
RUN cargo build --release

CMD ["sh", "-c", "cargo run --release -- /cuur/tests/main.toml"]

FROM opensuse:latest

RUN zypper update && zypper -y install curl build-essential pkg-config

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /cuur
COPY . .

RUN chmod +x scripts/build
RUN cargo build --release

CMD [ "cargo", "run", "--release", "--", "/cuur/tests/main.toml" ]

FROM opensuse/tumbleweed:latest

RUN zypper --gpg-auto-import-keys refresh && \
    zypper -n install curl gcc make pkg-config patterns-devel-base-devel_basis

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    echo 'export PATH="/root/.cargo/bin:$PATH"' >> /root/.bashrc

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /cuur
COPY . .

RUN chmod +x scripts/build

CMD ["cargo", "run", "--release", "--", "/cuur/tests/main.toml"]

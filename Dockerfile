FROM opensuse/tumbleweed

RUN zypper --non-interactive refresh && \
    zypper --non-interactive install -y \
        git curl tar gzip rpm-build dos2unix cargo \
        mingw64-cross-gcc mingw64-cross-binutils mingw64-filesystem \
        && zypper clean --all

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add x86_64-pc-windows-gnu && \
    cargo install cargo-rpm cargo-deb cargo-aur

WORKDIR /cuur
COPY . .

RUN dos2unix /cuur/scripts/build && chmod +x /cuur/scripts/build

ENTRYPOINT ["/cuur/scripts/build"]

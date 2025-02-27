FROM opensuse/tumbleweed

RUN zypper refresh && \
    zypper install -y \
        git \
        curl \
        tar \
        gzip \
        rpm-build \
        dos2unix \
        cargo \
        && zypper clean --all

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add x86_64-pc-windows-gnu && \
    cargo install cargo-rpm cargo-deb cargo-aur

WORKDIR /cuur
COPY . .

RUN dos2unix /cuur/scripts/build
RUN chmod +x /cuur/scripts/build

#CMD ["/cuur/scripts/build"]

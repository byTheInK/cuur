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

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    source $HOME/.cargo/env && \
    cargo install cargo-rpm cargo-deb cargo-aur \
    rustup target add x86_64-pc-windows-gnu
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /cuur

COPY . .

RUN dos2unix /cuur/scripts/build
RUN chmod +x /cuur/scripts/build

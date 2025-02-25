FROM debian:latest

RUN apt update && apt upgrade
RUN apt install curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

CMD [ "cargo", "build" ]
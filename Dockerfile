FROM rust:1.21.0

WORKDIR .
ADD . .

RUN apt-get update -qq
RUN apt-get install -y libdbus-1-3 libglib2.0-dev libgtk-3-dev
RUN cargo build --verbose --all

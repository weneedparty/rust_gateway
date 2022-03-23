FROM rust:1.59

RUN rustup component add rustfmt

RUN mkdir -p /code/src

WORKDIR /code

COPY ./Cargo.toml /code/Cargo.toml
COPY ./build.rs /code/build.rs
COPY ./src /code/src

RUN cargo build --bin server

EXPOSE 40054
EXPOSE 40055

CMD ["cargo", "run", "--bin", "server"]
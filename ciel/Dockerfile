
FROM rust:1.62

LABEL "Author"="cryoelite"

COPY src/ /usr/app/ciel/src/
COPY Cargo.toml /usr/app/ciel/
COPY Dockerfile /usr/app/ciel/

RUN cargo install --path /usr/app/ciel

CMD ["ciel"]
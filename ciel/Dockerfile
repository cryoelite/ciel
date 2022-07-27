
FROM rust:1.31

LABEL "Author"="cryoelite"
USER root

COPY . /usr/app/ciel

RUN cargo install --path /usr/app/ciel

CMD ["ciel"]
FROM rust
WORKDIR /usr/src/petermaniac-alert
COPY . .

RUN cargo install --path .
ENTRYPOINT ["petermaniac-alert"]
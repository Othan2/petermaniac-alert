FROM rust
WORKDIR /usr/src/petermaniac-alert
COPY . .

RUN cargo install --path .
CMD ["petermaniac-alert"]
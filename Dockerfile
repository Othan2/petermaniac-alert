FROM rust
WORKDIR /usr/src/petermaniac-alert
COPY . .

# you need to copy the following env vars into the image:
# GROUPME_TOKEN
# GROUPME_BOT_ID
# GROUPME_GROUP
# I would recommend doing that using a .env file and running using
# docker run --env-file=.env ...
RUN cargo install --path .
CMD ["petermaniac-alert"]
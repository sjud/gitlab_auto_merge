FROM rust:1.54-alpine
COPY . .
RUN apk add build-base && \
apk add openssl-dev && \
cargo install --path .
CMD ["gitlab_auto_merge"]
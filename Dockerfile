FROM rust:1.54-alpine
COPY . .
RUN cargo install --path .
CMD ["gitlab_auto_merge"]
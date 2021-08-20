FROM rust:latest
COPY . .
RUN cargo install --path .
CMD ["gitlab_auto_merge"]
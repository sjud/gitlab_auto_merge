FROM rust:1.54 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/gitlab_auto_merge /
ENTRYPOINT ["./gitlab_auto_merge"]

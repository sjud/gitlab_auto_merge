FROM rust:1.54-alpine as builder
WORKDIR /usr/src/gitlab_auto_merge
COPY . .
RUN apk add build-base && \
apk add openssl-dev && \
cargo install --path . && \
rm -R target

FROM alpine:latest
WORKDIR /usr/bin
COPY --from=builder /usr/src/gitlab_auto_merge/gitlab_auto_merge .
#ENV PATH=$PATH:/usr/src/gitlab_auto_merge
ENTRYPOINT ["gitlab_auto_merge"]

FROM rust:1.54-alpine3.14 as builder
WORKDIR /usr/src/gitlab_auto_merge
COPY . .
RUN apk add build-base && \
apk add openssl-dev && \
cargo install --path . --root . && \
rm -R target

FROM alpine:3.14
WORKDIR /usr/bin
COPY --from=builder /usr/src/gitlab_auto_merge/bin/gitlab_auto_merge .
#ENV PATH=$PATH:/usr/src/gitlab_auto_merge
ENTRYPOINT ["gitlab_auto_merge"]

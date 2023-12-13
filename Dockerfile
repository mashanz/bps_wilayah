# builder
FROM rust:1.74-bookworm AS base
WORKDIR /app
RUN update-ca-certificates
RUN apt update -y
RUN apt install build-essential -y
RUN apt install lld clang -y
FROM base AS builder
ENV USER=app
ENV UID=10001
RUN adduser --disabled-password --gecos "" --home "/nonexistent" --shell "/sbin/nologin" --no-create-home --uid "${UID}" "${USER}"
WORKDIR /app
COPY . .
RUN cargo build --release --locked
RUN strip -s target/release/bps_wilayah_api

# final outcome
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /app/target/release/bps_wilayah_api ./
EXPOSE 8080
CMD ["./bps_wilayah_api"]

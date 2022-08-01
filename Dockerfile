ARG BASE_IMAGE=rust:1.62-slim-buster

FROM $BASE_IMAGE as builder
WORKDIR app
COPY . .
RUN cargo build --release
CMD ["./target/release/rabbitmq-queue-alert"]

FROM $BASE_IMAGE
COPY --from=builder /app/target/release/rabbitmq-queue-alert /
COPY --from=builder /app/alerts.toml /
CMD ["./rabbitmq-queue-alert"]
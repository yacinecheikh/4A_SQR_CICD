FROM rust:1

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

WORKDIR /app

COPY . .

RUN cargo build
CMD ["cargo", "run"]

EXPOSE 8000
CMD ["rust-rocket-app"]

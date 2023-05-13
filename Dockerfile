# FROM rust:latest

# WORKDIR /usr/src/prueba

# COPY . .

# RUN cargo build --release

# CMD [ "cargo", "run" ]

###########################

# FROM rust:alpine3.16

# WORKDIR /usr/src/prueba
# COPY . .

# RUN cargo install --path .

# CMD ["prueba"]

###########################

FROM rust:1.69-buster AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# Production stage

FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/demo-actix-web .

# RUN cargo install --path .

CMD ["demo-actix-web"]
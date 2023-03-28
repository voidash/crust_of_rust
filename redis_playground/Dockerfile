FROM rust:latest

RUN apt-get update && apt-get install -y redis-server

# WORKDIR /app 

# COPY Cargo.toml Cargo.lock ./
# RUN cargo build --release

# COPY src ./src
# RUN cargo build --release

EXPOSE 6379

CMD ["redis-server", "--protected-mode no"]



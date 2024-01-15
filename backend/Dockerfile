##################
### BASE STAGE ###
##################
FROM rust:1.73 as base

# Install build dependencies
RUN cargo install --locked strip_cargo_version
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

###########################
### STRIP-VERSION STAGE ###
###########################
FROM base AS strip-version

COPY Cargo.lock Cargo.toml ./
COPY server/Cargo.toml ./server/
COPY game/Cargo.toml ./game/
COPY db/Cargo.toml ./db/
COPY websocket/Cargo.toml ./websocket/
COPY demo_game_creator/Cargo.toml ./demo_game_creator/
RUN strip_cargo_version

###################
### BUILD STAGE ###
###################
FROM base AS build

# compile dependencies first so that Docker caches them
RUN cargo init --bin server/
RUN cargo init --lib game/
RUN cargo init --lib db/
RUN cargo init --lib websocket/
RUN cargo init --bin demo_game_creator/ 

COPY --from=strip-version /app/ /app/
RUN VERIFY_DEMO_GAMES=false cargo build --release --target x86_64-unknown-linux-musl

# compile the app itself
COPY . .
RUN VERIFY_DEMO_GAMES=false cargo build --release --target x86_64-unknown-linux-musl
RUN strip /app/target/x86_64-unknown-linux-musl/release/ti-helper-server

########################
### PRODUCTION STAGE ###
########################
FROM scratch

ENV RUST_LOG="info"

WORKDIR /

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/ti-helper-server /
COPY db/migrations /

CMD ["/ti-helper-server"]


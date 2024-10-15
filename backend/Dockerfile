##################
### BASE STAGE ###
##################
FROM rust:1.81 as base

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
COPY game_logic/Cargo.toml ./game_logic/
COPY game_data/Cargo.toml ./game_data/
COPY db/Cargo.toml ./db/
COPY websocket/Cargo.toml ./websocket/
COPY demo_game_creator/Cargo.toml ./demo_game_creator/
COPY map_render/Cargo.toml ./map_render/
COPY milty/Cargo.toml ./milty/

RUN strip_cargo_version

###################
### BUILD STAGE ###
###################
FROM base AS build

# compile dependencies first so that Docker caches them
RUN cargo init --bin server/
RUN cargo init --lib game_data/
RUN cargo init --lib db/
RUN cargo init --lib websocket/
RUN cargo init --bin demo_game_creator/ 
RUN cargo init --bin map_render/
RUN cargo init --lib milty/
RUN cargo init --lib game_logic

COPY --from=strip-version /app/ /app/

RUN apt-get update && apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-dev musl-tools

#RUN VERIFY_DEMO_GAMES=false cargo build --release --target x86_64-unknown-linux-musl

# compile the app itself
COPY . .
RUN VERIFY_DEMO_GAMES=false cargo build --release --target x86_64-unknown-linux-musl --bin ti_helper_server
RUN strip /app/target/x86_64-unknown-linux-musl/release/ti_helper_server

########################
### PRODUCTION STAGE ###
########################
FROM scratch

ENV RUST_LOG="info"

WORKDIR /

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/ti_helper_server /
COPY db/migrations /

CMD ["/ti_helper_server"]


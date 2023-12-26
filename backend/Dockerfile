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
RUN strip_cargo_version

###################
### BUILD STAGE ###
###################
FROM base AS build

# compile dependencies first so that Docker caches them
RUN cargo init --bin .
COPY --from=strip-version /app/Cargo.toml /app/Cargo.lock /app/
RUN cargo build --release --target x86_64-unknown-linux-musl

# compile the app itself
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip /app/target/x86_64-unknown-linux-musl/release/ti-assistant

########################
### PRODUCTION STAGE ###
########################
FROM scratch

ENV RUST_LOG="info"

WORKDIR /

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/ti-assistant /
COPY migrations /

CMD ["/ti-assistant"]


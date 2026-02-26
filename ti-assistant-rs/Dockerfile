FROM rust:1.93 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner

COPY ./libs ./libs
COPY ./packages ./packages
COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Install required build deps
RUN apt-get update
RUN apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0

RUN cargo chef cook --release --recipe-path recipe.json

COPY ./libs ./libs
COPY ./packages ./packages
COPY ./Cargo.toml .
COPY ./Cargo.lock .

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force --version 0.7.2
ENV PATH="/.cargo/bin:$PATH"

RUN dx bundle --release --package web

#FROM scratch AS runtime
FROM chef AS runtime
COPY --from=builder /app/target/dx/web/release/web/ /usr/local/app/

ENV PORT=8080
ENV IP=0.0.0.0
ENV RUST_LOG=info

EXPOSE 8080

WORKDIR /usr/local/app
RUN ls -lah /usr/local/app
ENTRYPOINT [ "/usr/local/app/web" ]

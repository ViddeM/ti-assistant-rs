FROM rust:1.91 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:PATH"

RUN dx bundle --web --release

FROM scratch AS runtime
COPY --from=builder /app/target/dx/ti_assistant_rs/release/web/ /usr/local/app

ENV PORT=8080
ENV IP=0.0.0.0
ENV RUST_LOG=info

EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/urs/local/app/server" ]

##############
# base image #
##############
FROM rust:1.70.0 AS chef

WORKDIR /app

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN rustup component add \
        clippy \
        rustfmt && \
    cargo binstall --no-confirm --no-symlinks cargo-chef && \
    cargo binstall --no-confirm --no-symlinks sqlx-cli --version 0.6.3

###########
# planner #
###########
FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

########
# main #
########
FROM chef

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --recipe-path recipe.json

COPY . .

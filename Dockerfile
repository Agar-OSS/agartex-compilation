FROM rust:1.68-slim-buster as builder
WORKDIR /app/src

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

# Force crates.io init for better docker caching
COPY docker/caching.rs src/main.rs
COPY Cargo.lock .
COPY Cargo.toml .
RUN cargo build --release

COPY . .

# Force cargo to recompile
RUN touch src/main.rs
RUN cargo build --release

FROM agaross.azurecr.io/agar-oss/latex-base:latest as environment

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
VOLUME /app/blobs

COPY --from=builder /app/src/target/release/agartex-compilation .

# Test
RUN mkdir tex
COPY example.tex tex/example.tex
RUN cd tex && \
    latexmk -pdf example.tex && \
    cd ..

EXPOSE 3300
ENTRYPOINT [ "./agartex-compilation" ]

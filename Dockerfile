# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM rust:1.94-alpine AS builder

# Instala dependências de compilação para Alpine
RUN apk add --no-cache musl-dev gcc

WORKDIR /src
COPY . .

# Usa o Cache Mount para acelerar o download e compilação das crates
RUN --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/src/target \
  RUSTFLAGS='-C target-feature=+crt-static' \
  cargo build --release --target x86_64-unknown-linux-musl && \
  cp target/x86_64-unknown-linux-musl/release/optirust /usr/local/bin/optirust

# Imagem Final (Distroless/Scratch)
FROM scratch
LABEL org.opencontainers.image.title="OptiRust"
LABEL org.opencontainers.image.description="High-performance PNG optimizer CLI"

# Copia apenas o binário estático do builder
COPY --from=builder /usr/local/bin/optirust /optirust

# Define o binário como ponto de entrada único
ENTRYPOINT [ "/optirust" ]

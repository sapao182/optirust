# syntax=docker/dockerfile:1
FROM --platform=$BUILDPLATFORM rust:1.94-alpine AS builder

# Instala o essencial para compilação estática
RUN apk add --no-cache musl-dev gcc

WORKDIR /src
COPY . .

# Usa o Cache Mount para acelerar o download e compilação das crates
RUN --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/src/target \
  RUSTFLAGS='-C target-feature=+crt-static' \
  cargo build --release --target x86_64-unknown-linux-musl && \
  cp target/release/optirust /usr/local/bin/optirust

# Imagem final
FROM scratch
LABEL org.opencontainers.image.title="OptiRust"
LABEL org.opencontainers.image.description="High-performance PNG optimizer CLI"

# Copia apenas o binário estático
COPY --from=builder /usr/local/bin/optirust ./optirust

EXPOSE 8000
CMD ["./optirust"]
# No scratch, o path precisa ser absoluto no Entrypoint
# ENTRYPOINT [ "/optirust" ]
# CMD [ "--help" ]

# 🦀 OptiRust: PNG Multi-threaded Optimizer
O **OptiRust** é uma ferramenta de linha de comando (CLI) desenvolvida em **Rust** focada na otimização em massa de imagens PNG. O projeto foi construído aplicando conceitos avançados de sistemas, como processamento paralelo, serialização rigorosa e desenvolvimento guiado por testes (TDD).

> [!WARNING]
> Os arquivos otimizados sobreescrevem os originais!

## 🚀 Diferenciais Técnicos
- **Processamento Paralelo (Rayon):** Diferente de scripts sequenciais, o OptiRust utiliza um pool de threads para processar múltiplas imagens simultaneamente, escalando a performance de acordo com os núcleos da CPU.

- **Arquitetura Robusta:** Separação clara entre o motor de compressão (`optimizer`), o rastreador de arquivos (`scanner`) e o gerador de métricas (`report`).

- **Segurança de Memória:** Aproveita o sistema de _ownership_ do Rust para garantir que a manipulação de arquivos e buffers de memória seja livre de _data races_.

- **Configuração Flexível:** Suporte a arquivos `optirust.toml` para definição de níveis de compressão e persistência de preferências.

## 🛠️ Tecnologias Utilizadas
- **Engine:** `oxipng` (Motor de compressão de alto desempenho).

- **CLI & UX:** `clap` (Parsing de argumentos) e `indicatif` (Barras de progresso dinâmicas).

- **Paralelismo:** `rayon` (Data parallelism library).

- **Data Handling:** `serde` (JSON/TOML) e `chrono` (Timestamping).

- **Estilização:** `colored` (ANSI terminal colors).

## 📦 Como Instalar e Rodar
**Pré-requisitos**

- Rust (Cargo) 1.70+

- Ambiente Linux (Homologado em Arch Linux)

**Compilação de Alta Performance**

Para obter o máximo de desempenho do paralelismo, compile sempre em modo release:

```Bash
cargo build --release
```

**Comandos Principais**

Inicializar Configuração:

```Bash
./target/release/optirust init
```

**Executar Otimização com Relatório Visual**

```Bash
./target/release/optirust run ./assets --summary
```

### 📊 Exemplo de Relatório (JSON)
Ao final de cada execução, o sistema gera um `optirust_report.json` detalhado para auditoria:

```JSON
{
  "timestamp": "2026-03-17T14:20:00Z",
  "status": "success",
  "summary": {
    "files_processed": 12,
    "total_original_size_kb": 4500.5,
    "total_optimized_size_kb": 3200.2,
    "space_saved_kb": 1300.3,
    "efficiency_gain_percent": 28.9
  },
  "files": [
    {
      "name": "hero-banner.png",
      "path": "./assets/hero-banner.png",
      "original_kb": 1200.0,
      "optimized_kb": 850.5,
      "saved_kb": 349.5,
      "ratio": "29.1%"
    }
  ]
}
```

## 🐳 Uso via Docker (DevOps Ready)
O OptiRust está disponível como uma imagem Docker ultra-leve (baseada em scratch), ideal para ser integrada em pipelines de CI/CD para otimização automática de assets.

### 1. Rodando a imagem do Docker Hub

```Bash
# Otimizando uma pasta local usando o container
docker run --rm -v $(pwd):/data betoxvt/optirust run /data --summary
```

### 2. Construindo a imagem localmente
Utilizamos um processo de Multi-stage build para garantir que a imagem final contenha apenas o binário estático:

```Bash
docker build -t optirust .
```

### 3. Integração em Pipeline (Exemplo GitHub Actions)
Você pode usar o OptiRust para otimizar imagens antes do deploy:

```YAML
- name: Optimize PNGs
  run: |
    docker run --rm -v ${{ github.workspace }}/assets:/assets \
    betoxvt/optirust run /assets
```

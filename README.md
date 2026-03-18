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

## 🏗️ Arquitetura do Projeto

```
├── src/
│   ├── main.rs             # Entrada do programa
│   ├── config.rs           # Configuração e Persistência
│   ├── report.rs           # Gerador de Relatórios
│   ├── scanner.rs          # Rastreador de Arquivos
│   └── optimizer.rs        # Motor de Compressão
├── examples/               # Scripts criados durante o desenvolvimento 
├── assets/                 # Imagens para testar
├── Cargo.toml              # Configuração do Cargo
├── Cargo.lock              # Lockfile do Cargo
├── Dockerfile              # Arquivo Docker
├── optirust_report.json    # Relatório real
├── optirust.toml           # Configuração padrão
├── test_input.png          # Imagem para testes unitários
└── README.md               # Documentação
```

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
docker run --rm -t -v $(pwd)/assets:/data betoxvt/optirust:latest run /data --summary
```
> [!TIP]
> Use a flag `-t` do `docker run` para ver o relatório colorido!

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
    betoxvt/optirust:latest run /assets
```
## ✅ Funcionalidades Implementadas (v0.1.0)

- [x] **Core Engine em Rust:** Processamento de alta performance utilizando a Edition 2024.
- [x] **Paralelismo com Rayon:** Otimização multi-thread para processamento em massa de diretórios.
- [x] **Interface CLI Intuitiva:** Comandos estruturados via `Clap` com suporte a ajuda detalhada (`--help`).
- [x] **Relatórios Dinâmicos:** Geração automática de `optirust_report.json` e resumo visual colorido no terminal.
- [x] **Containerização Estática:** Imagem Docker ultra-leve baseada em `scratch` (<3MB), garantindo portabilidade total.
- [x] **Cálculo de Eficiência:** Métricas precisas de ganho de espaço por arquivo e total da operação.

## 🗺️ Roadmap de Desenvolvimento

Abaixo estão as funcionalidades planejadas para as próximas iterações do OptiRust:

- [ ] **Modo Silent (`--silent`):** Redução de logs para integração limpa em CI/CD.
- [ ] **Configuração Dinâmica:** Suporte a `--config <path>` e argumentos de linha de comando para sobrescrever o `optirust.toml`.
- [ ] **Suporte Multi-Formato:** Expansão do algoritmo de compressão para:
    - [ ] 🖼️ **Imagens:** WebP, JPEG e SVG.
    - [ ] 📄 **Documentos:** Otimização de camadas de PDF.
- [ ] **Internacionalização (i18n):** Suporte a logs e relatórios em Inglês e Português.

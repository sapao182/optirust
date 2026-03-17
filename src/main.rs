mod config;
mod optimizer;
mod report;
mod scanner;

use clap::{Parser, Subcommand};
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "OptiRust", about = "Optimizador de imagens PNG")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inicia o processo de otimização
    Run {
        /// O diretório contendo os PNGs
        path: PathBuf,

        /// Exibe o resumo visual do relatório no terminal
        #[arg(short, long, default_value_t = false)]
        summary: bool,
    },
    /// Gera um arquivo de configurações padrão
    Init,
}

fn main() {
    let cli = Cli::parse();
    let settings = config::load_config();

    // Definição de Cores ANSI
    let green = "\x1b[32m";
    let blue = "\x1b[34m";
    let yellow = "\x1b[33m";
    let reset = "\x1b[0m";

    match cli.command {
        Commands::Run { path, summary } => {
            println!(
                "🛠️ Otimizando em nível: {}{}{}",
                green, settings.level, reset
            );
            println!("{}🔍 Varrendo diretório: {:?}{}", blue, path, reset);

            let start_time = Instant::now();

            // 1. Scanner
            let files = scanner::find_png_files(path);
            if files.is_empty() {
                println!("{}⚠️ Nenhum arquivo PNG encontrado.{}", yellow, reset);
                return;
            }

            println!("{}📦 Encontrados: {} arquivos.{}", blue, files.len(), reset);

            let pb = ProgressBar::new(files.len() as u64);

            // 2. Optimizer + Rayon
            let results: Vec<_> = files
                .par_iter()
                .map(|file| {
                    let res = optimizer::optimize_png(file);
                    pb.inc(1);
                    res
                })
                .collect();

            pb.finish_and_clear();

            // 3. Preparação das métricas para o Relatório
            let report_data: Vec<(PathBuf, usize, usize)> = files
                .into_iter()
                .zip(results)
                .filter_map(|(path, res)| match res {
                    Ok((orig, optim)) => Some((path, orig, optim)),
                    Err(e) => {
                        eprintln!("{}", e);
                        None
                    }
                })
                .collect();

            // 4. Geração do Relatório
            match report::generate_json_report(report_data) {
                Ok(full_report) => {
                    if summary {
                        report::print_terminal_summary(&full_report);
                    }
                    let duration = start_time.elapsed();
                    println!("✅ Concluído em {:?}!", duration);
                    println!("📝 Relatório detalhado gerado em 'optirust_report.json'");
                }
                Err(e) => eprintln!("Erro ao gerar relatório: {}", e),
            }
        }

        Commands::Init => {
            println!("{}🛠️ Gerando arquivo de configuração...{}", blue, reset);
            match config::create_default_config() {
                Ok(_) => println!(
                    "{}[OK]{} Arquivo 'optirust.toml criado com sucesso!",
                    green, reset
                ),
                Err(e) => eprintln!("Erro: {}", e),
            }
        }
    }
}

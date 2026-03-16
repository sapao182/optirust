mod config;
mod optimizer;
mod scanner;

use clap::{Parser, Subcommand};
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
    },
    /// Gera um arquivo de configurações padrão
    Init,
}

fn main() {
    let cli = Cli::parse();
    let settings = config::load_config();

    match cli.command {
        Commands::Run { path } => {
            println!("Otimizando em nível: {}", settings.level);

            println!(
                "Iniciando busca de arquivos no diretório: {}",
                path.display()
            );
            let start_time = Instant::now();

            // 1. Scanner (Busca de arquivos)
            let files = scanner::find_png_files(path);
            println!("Encontrados {} arquivos.", files.len());

            // 2. Optimizer + Rayon
            let results: Vec<_> = files.par_iter().map(optimizer::optimize_png).collect();

            // 3. Resumo simples
            let duration = start_time.elapsed();
            let success_count = results.iter().filter(|result| result.is_ok()).count();
            let failed_count = results.len() - success_count;
            println!("Processamento concluído em: {:.2?}", duration);
            println!("Sucessos: {} | Falhas: {}", success_count, failed_count);
        }

        Commands::Init => {
            match config::create_default_config() {
                Ok(_) => println!("Arquivo 'optirust.toml criado com sucesso!"),
                Err(e) => eprintln!("Erro: {}", e),
            }
            println!("Gerando arquivo de configuração...")
        }
    }
}

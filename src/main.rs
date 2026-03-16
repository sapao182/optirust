mod optimizer;
mod scanner;

use rayon::prelude::*;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    let path = PathBuf::from("./test_folder");

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

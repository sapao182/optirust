use std::thread;
use std::time::Duration;

fn main() {
    // Definição de Cores ANSI
    let green = "\x1b[32m";
    let blue = "\x1b[34m";
    let yellow = "\x1b[33m";
    let red = "\x1b[31m";
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";

    println!("{}🔍 Varrendo diretório: ./assets{}", blue, reset);
    thread::sleep(Duration::from_millis(500));
    println!("{}📦 Encontrados: 4 arquivos .png{}", blue, reset);

    // Simulação de Barra de Progresso Simples
    print!("🚀 Otimizando: ");
    for _ in 0..20 {
        print!("█");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!(" 100%\n");

    // Tabela de Resultados
    println!(
        "{}----------------------------------------------------------------------{}",
        bold, reset
    );
    println!("{}✨ OPTIRUST - RELATÓRIO DE OTIMIZAÇÃO ✨{}", bold, reset);
    println!(
        "{}----------------------------------------------------------------------{}",
        bold, reset
    );
    println!(
        "{}STATUS      ARQUIVO                     ORIGINAL    OTIMIZADO   GANHO{}",
        bold, reset
    );
    println!("----------------------------------------------------------------------");

    println!(
        "{}[SUCCESS]{}   ./hero-banner.png           1200.0 KB   850.5 KB    {}[ 29.1% ]{}",
        green, reset, green, reset
    );
    println!(
        "{}[SUCCESS]{}   ./logo-footer.png           150.2 KB    90.1 KB     {}[ 40.0% ]{}",
        green, reset, green, reset
    );
    println!(
        "{}[SKIPPED]{}   ./icon-check.png            12.5 KB     12.5 KB     {}[  0.0% ]{}",
        yellow, reset, yellow, reset
    );
    println!(
        "{}[ERROR]{}     ./corrupted-img.png         ----        ----        {}[ FAIL  ]{}",
        red, reset, red, reset
    );

    println!("----------------------------------------------------------------------");
    println!("\n{}📊 RESUMO DA OPERAÇÃO:{}", bold, reset);
    println!("{}🔹 Arquivos processados: 4{}", blue, reset);
    println!("{}🔹 Economia total:      422.1 KB{}", blue, reset);
    println!("{}🔹 Ganho de eficiência:  31.2%{}", green, reset);

    println!("\n{}✅ Otimização concluída com sucesso!{}", green, reset);
}

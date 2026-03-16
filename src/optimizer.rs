use oxipng::{InFile, Options, OutFile, optimize};
use std::path::PathBuf;

/// Otimiza uma imagem PNG individual
pub fn optimize_png(path: &PathBuf) -> Result<(), String> {
    // Configurações padrão do oxipng (Equivalente ao nível 2)
    let options = Options::default();

    // Define a entrada e saída (neste caso, sobrescreve o arquivo original)
    let input = InFile::Path(path.to_path_buf());
    let output = OutFile::Path {
        path: Some(path.to_path_buf()),
        preserve_attrs: false,
    };

    // Executa a otimização e trata o erro de forma idiomática
    optimize(&input, &output, &options)
        .map(|_| ())
        .map_err(|e| format!("Erro ao otimizar {:?}: {}", path, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_compression_reduces_size() {
        // 1. Preparar: Copia a imagem de teste para não estragar a original
        let input_path = PathBuf::from("test_input.png");
        let test_path = PathBuf::from("test_output.png");

        // Verifica se a imagem de teste existe antes de começar
        if !input_path.exists() {
            panic!("Por favor, adicione a imagem 'test_input.png' na raiz do projeto.");
        }

        fs::copy(&input_path, &test_path).unwrap();

        // 2. Medir o tamanho inicial
        let initial_size = fs::metadata(&test_path).unwrap().len();

        // 3. Executar a compressão
        optimize_png(&test_path).expect("A compressão falhou");

        // 4. Medir o tamanho final
        let final_size = fs::metadata(&test_path).unwrap().len();

        // 5. Validar: O tamanho final deve ser menor ou igual ao inicial
        println!(
            "Inicial: {} bytes | Final: {} bytes",
            initial_size, final_size
        );
        assert!(final_size < initial_size);

        // Limpeza: Remove a imagem de teste
        fs::remove_file(test_path).unwrap();
    }
}

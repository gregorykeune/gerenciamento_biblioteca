// ************* ATENÇãO *******************
// rodar cargo run no diretorio projeto2 (:



mod biblioteca;
mod errors;
mod traits;

use biblioteca::Biblioteca;
use std::path::Path;

fn main() {
    // Caminho do arquivo JSON de persistência
    let caminho_arquivo = Path::new("dados_biblioteca.json");

    // Tenta carregar a biblioteca existente, ou cria nova se o arquivo não existir
    let mut biblioteca = match Biblioteca::carregar(caminho_arquivo) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Erro ao carregar biblioteca: {:?}", e);
            println!("Criando nova biblioteca...");
            Biblioteca::nova(caminho_arquivo)
        }
    };

    loop {
        println!("\n===== MENU BIBLIOTECA =====");
        println!("[1] Adicionar livro");
        println!("[2] Adicionar usuário");
        println!("[3] Registrar empréstimo");
        println!("[4] Listar livros");
        println!("[5] Listar usuários");
        println!("[6] Listar empréstimos");
        println!("[7] Salvar e sair");

        let opcao = biblioteca::ler_i32("Escolha uma opção: ".to_string());

        match opcao {
            1 => {
                if let Err(e) = biblioteca.adicionar_livro() {
                    println!("Erro ao adicionar livro: {:?}", e);
                }
            }
            2 => {
                if let Err(e) = biblioteca.adicionar_usuario() {
                    println!("Erro ao adicionar usuário: {:?}", e);
                }
            }
            3 => {
                if let Err(e) = biblioteca.registrar_emprestimo() {
                    println!("Erro ao registrar empréstimo: {:?}", e);
                }
            }
            4 => biblioteca.listar_livros(),
            5 => biblioteca.listar_usuarios(),
            6 => biblioteca.listar_emprestimos(),
            7 => {
                if let Err(e) = biblioteca.salvar() {
                    println!("Erro ao salvar biblioteca: {:?}", e);
                } else {
                    println!("Biblioteca salva com sucesso. Saindo...");
                }
                break;
            }
            _ => println!("Opção inválida!"),
        }
    }
}

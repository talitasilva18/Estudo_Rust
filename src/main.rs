mod models;
mod search;
mod utils;

use search::{indexar_produtos, buscar};
use utils::produtos_exemplo;

use std::io::{self, Write};

fn main() {
    let produtos = produtos_exemplo();
    let indice = indexar_produtos(&produtos);

    println!("Sistema de Busca MegaStore 🚀");
    println!("Digite um termo para buscar produtos (ex: celular, Nike, eletrônico):");

    print!("> ");
    io::stdout().flush().unwrap();

    let mut entrada = String::new();
    if let Ok(_) = io::stdin().read_line(&mut entrada) {
        let termo = entrada.trim();

        println!("\nBuscando por '{}':", termo);

        match buscar(&indice, termo) {
            Some(resultados) if !resultados.is_empty() => {
                for produto in resultados {
                    println!("- [{}] {} ({})", produto.id, produto.nome, produto.marca);
                }
            }
            _ => {
                println!("Nenhum produto correspondente foi encontrado para o termo '{}'.", termo);
            }
        }
    } else {
        println!("Erro ao ler a entrada.");
    }
}

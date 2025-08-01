use crate::models::Produto;

pub fn produtos_exemplo() -> Vec<Produto> {
    vec![
        Produto {
            id: 1,
            nome: "Celular Galaxy A12".to_string(),
            marca: "Samsung".to_string(),
            categoria: "Eletrônicos".to_string(),
        },
        Produto {
            id: 2,
            nome: "Notebook Aspire 5".to_string(),
            marca: "Acer".to_string(),
            categoria: "Informática".to_string(),
        },
        Produto {
            id: 3,
            nome: "Tênis Esportivo".to_string(),
            marca: "Nike".to_string(),
            categoria: "Vestuário".to_string(),
        },
    ]
}

pub fn gerar_termos(produto: &Produto) -> Vec<String> {
    let mut termos = vec![];
    termos.extend(produto.nome.split_whitespace().map(|s| s.to_lowercase()));
    termos.push(produto.marca.to_lowercase());
    termos.push(produto.categoria.to_lowercase());
    termos
}

use crate::models::Produto;
use crate::utils::gerar_termos;
use std::collections::HashMap;

pub fn indexar_produtos(produtos: &[Produto]) -> HashMap<String, Vec<Produto>> {
    let mut indice = HashMap::new();

    for produto in produtos {
        let termos = gerar_termos(produto);
        for termo in termos {
            indice.entry(termo)
                .or_insert_with(Vec::new)
                .push(produto.clone());
        }
    }

    indice
}

pub fn buscar<'a>(indice: &'a HashMap<String, Vec<Produto>>, termo: &str) -> Option<&'a Vec<Produto>> {
    indice.get(&termo.to_lowercase())
}

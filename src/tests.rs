#[cfg(test)]
mod tests {
    use crate::models::Produto;
    use crate::search::{indexar_produtos, buscar};

    #[test]
    fn test_busca_existente() {
        let produtos = vec![Produto {
            id: 1,
            nome: "Celular Galaxy A12".to_string(),
            marca: "Samsung".to_string(),
            categoria: "Eletrônicos".to_string(),
        }];

        let indice = indexar_produtos(&produtos);
        let resultado = buscar(&indice, "celular");

        assert!(resultado.is_some());
        assert_eq!(resultado.unwrap().len(), 1);
    }

    #[test]
    fn test_busca_inexistente() {
        let produtos = vec![];
        let indice = indexar_produtos(&produtos);
        let resultado = buscar(&indice, "inexistente");

        assert!(resultado.is_none());
    }
}

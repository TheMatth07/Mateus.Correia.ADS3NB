fn main() {
    println!("Aqui está sua arvore!")
}


// Arvore binária de busca
#[derive(Debug)]
struct BST {
    root: Option<Box<Node>>, 
}

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// Testes para a implementação da árvore binária de busca (BST)
#[cfg(test)]
mod bst_tests {
    use crate::BST;

    #[test]
    fn test_bst_new_and_empty() {
        // Teste 1: Criar uma nova árvore e verificar se está vazia
        let bst = BST::new();
        assert!(bst.is_empty());
    }

    #[test]
    fn test_bst_insert_and_search() {
        // Teste 2: Inserir elementos e verificar se estão na árvore
        let mut bst = BST::new();
        
        // Inserir alguns valores
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        
        // Verificar se os valores inseridos estão na árvore
        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));
        
        // Verificar que um valor não inserido não está na árvore
        assert!(!bst.search(20));
        
        // A árvore não deve mais estar vazia
        assert!(!bst.is_empty());
    }
}

impl BST {
    // Criar uma nova árvore vazia
    fn new() -> Self {
        BST { root: None }
    }

    // Verificar se a árvore está vazia
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    // Inserir valor
    fn insert(&mut self, value: i32) {
        let mut current = &mut self.root;

        // Loop para encontrar o lugar certo para inserir
        loop {
            match current {
                // Lugar vazio encontrado, inserimos aqui
                None => {
                    *current = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }));
                    return;
                }
                // Navegamos para a posição correta
                Some(node) => {
                    if value < node.value {
                        // Ir para esquerda
                        current = &mut node.left;
                    } else if value > node.value {
                        // Ir para direita
                        current = &mut node.right;
                    } else {
                        // Valor duplicado, nada a fazer
                        return;
                    }
                }
            }
        }
    }

    // Buscar valor
    fn search(&self, value: i32) -> bool {
        let mut current = &self.root;

        // Loop para navegar até encontrar ou não o valor
        while let Some(node) = current {
            if value == node.value {
                return true; // Encontrou
            } else if value < node.value {
                current = &node.left; // Vai para esquerda
            } else {
                current = &node.right; // Vai para direita
            }
        }

        false // Não encontrou
    }
}

// ;)
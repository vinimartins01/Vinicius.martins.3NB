// Testes Simples para Implementação de Árvore Binária de Busca (BST)
// Os alunos devem implementar a árvore para fazer estes testes passarem

#[cfg(test)]
mod bst_tests {
    use crate::BST;
    // Importe sua implementação de BST aqui
    // use crate::BST;
    
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

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub struct BST {
    pub root: Option<Box<Node>>,
}

impl BST {
    // Criar uma nova árvore vazia
    pub fn new() -> Self {
        BST { root: None }
    }

    // Verificar se a árvore está vazia
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    // Inserir um valor na árvore
    pub fn insert(&mut self, value: i32) {
        let new_node = Box::new(Node { value, left: None, right: None });
        if self.root.is_none() {
            self.root = Some(new_node);
        } else {
            BST::insert_node(self.root.as_mut().unwrap(), new_node);
        }
    }

    fn insert_node(current: &mut Box<Node>, new_node: Box<Node>) {
        if new_node.value < current.value {
            if current.left.is_none() {
                current.left = Some(new_node);
            } else {
                BST::insert_node(current.left.as_mut().unwrap(), new_node);
            }
        } else {
            if current.right.is_none() {
                current.right = Some(new_node);
            } else {
                BST::insert_node(current.right.as_mut().unwrap(), new_node);
            }
        }
    }

    // Buscar um valor na árvore
    pub fn search(&self, value: i32) -> bool {
        BST::search_node(self.root.as_ref(), value)
    }

    fn search_node(current: Option<&Box<Node>>, value: i32) -> bool {
        match current {
            Some(node) => {
                if node.value == value {
                    true
                } else if value < node.value {
                    BST::search_node(node.left.as_ref(), value)
                } else {
                    BST::search_node(node.right.as_ref(), value)
                }
            }
            None => false,
        }
    }
}


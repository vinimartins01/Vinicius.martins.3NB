pub struct Stack<T> {
    elementos: Vec<T>,
    capacidade_maxima: Option<usize>,
}
impl<T> Stack<T> {
    // A implementação será adicionada após criar os testes
    // Cria uma nova pilha sem limite de capacidade
    pub fn nova() -> Self {
        Stack {
            elementos: Vec::new(), // Inicializa um vetor vazio
            capacidade_maxima: None, // Sem limite de capacidade
        }
    }

    // Cria uma nova pilha com uma capacidade máxima definida
    pub fn nova_com_capacidade(capacidade: usize) -> Self {
        Stack {
            elementos: Vec::with_capacity(capacidade), // Inicializa um vetor com capacidade pré-alocada
            capacidade_maxima: Some(capacidade), // Define o limite de capacidade
        }
    }

    // Verifica se a pilha está vazia
    pub fn esta_vazia(&self) -> bool {
        self.elementos.is_empty()
    }

    // Retorna o número de elementos na pilha
    pub fn tamanho(&self) -> usize {
        self.elementos.len()
    }

    // Adiciona um elemento ao topo da pilha
    pub fn empilhar(&mut self, item: T) -> Result<(), &'static str> {
        if let Some(cap) = self.capacidade_maxima { // Verifica se há um limite de capacidade
            if self.elementos.len() >= cap {
                return Err("Capacidade máxima atingida"); // Retorna erro se a pilha estiver cheia
            }
        }
        self.elementos.push(item); // Adiciona o elemento ao topo da pilha
        Ok(())
    }

    // Remove e retorna o elemento do topo da pilha
    pub fn desempilhar(&mut self) -> Option<T> {
        self.elementos.pop() // Remove e retorna o último elemento, se existir
    }

    // Retorna uma referência ao elemento do topo da pilha sem removê-lo
    pub fn topo(&self) -> Option<&T> {
        self.elementos.last()
    }

    // Remove todos os elementos da pilha
    pub fn limpar(&mut self) {
        self.elementos.clear();
    }

    // Verifica se a pilha está cheia (caso tenha uma capacidade definida)
    pub fn esta_cheia(&self) -> bool {
        match self.capacidade_maxima {
            Some(cap) => self.elementos.len() >= cap, // Se a capacidade foi definida, compara com o tamanho atual
            None => false, // Se não há limite de capacidade, nunca está cheia
        }
    }
    
}
#[test]
fn nova_pilha_esta_vazia() {
    let pilha: Stack<i32> = Stack::nova();
    assert!(pilha.esta_vazia());
}
#[test]
fn nova_pilha_com_capacidade() {
    let pilha: Stack<i32> = Stack::nova_com_capacidade(5);
    assert!(pilha.esta_vazia());
    assert_eq!(pilha.tamanho(), 0);
}
#[test]
fn topo_retorna_ultimo_elemento() {
    let mut pilha = Stack::nova();
    pilha.empilhar(1).unwrap();
    pilha.empilhar(2).unwrap();
    pilha.empilhar(3).unwrap();
    assert_eq!(*pilha.topo().unwrap(), 3);
    assert_eq!(pilha.tamanho(), 3);
}
#[test]
fn desempilhar_retorna_ultimo_elemento() {
    let mut pilha = Stack::nova();
    pilha.empilhar(1).unwrap();
    pilha.empilhar(2).unwrap();
    let valor = pilha.desempilhar();
    assert_eq!(valor, Some(2));
    assert_eq!(pilha.tamanho(), 1);
}
#[test]
fn desempilhar_pilha_vazia_retorna_none() {
    let mut pilha: Stack<i32> = Stack::nova();
    let valor = pilha.desempilhar();
    assert_eq!(valor, None);
}
#[test]
fn topo_pilha_vazia_retorna_none() {
    let pilha: Stack<i32> = Stack::nova();
    let valor = pilha.topo();
    assert_eq!(valor, None);
}
#[test]
fn pilha_limitada_rejeita_elementos_alem_da_capacidade() {
    let mut pilha = Stack::nova_com_capacidade(2);
    assert!(pilha.empilhar(1).is_ok());
    assert!(pilha.empilhar(2).is_ok());
    assert!(pilha.empilhar(3).is_err());
    assert_eq!(pilha.tamanho(), 2);
}
#[test]
fn pilha_ilimitada_aceita_muitos_elementos() {
    let mut pilha = Stack::nova();
    for i in 0..1000 {
        assert!(pilha.empilhar(i).is_ok());
    }
    assert_eq!(pilha.tamanho(), 1000);
}
#[test]
fn limpar_remove_todos_elementos() {
    let mut pilha = Stack::nova();
    pilha.empilhar(1).unwrap();
    pilha.empilhar(2).unwrap();
    pilha.empilhar(3).unwrap();
    pilha.limpar();
    assert!(pilha.esta_vazia());
    assert_eq!(pilha.tamanho(), 0);
}
#[test]
fn esta_cheia_retorna_correto() {
    let mut pilha = Stack::nova_com_capacidade(2);
    assert!(!pilha.esta_cheia());
    pilha.empilhar(1).unwrap();
    assert!(!pilha.esta_cheia());
    pilha.empilhar(2).unwrap();
    assert!(pilha.esta_cheia());
}
#[test]
fn pilha_ilimitada_nunca_esta_cheia() {
    let mut pilha = Stack::nova();
    for i in 0..100 {
        pilha.empilhar(i).unwrap();
        assert!(!pilha.esta_cheia());
    }
}

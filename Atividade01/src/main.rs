//Usando a biblioteca padrão
use std::collections::VecDeque;

fn main () {
    let mut fila: VecDeque<i32> = VecDeque::new();

    //Adiconando elementos na fila
    println!("A implementação da fila FIFO usando VecDeque");


    fila.push_back(07);
    fila.push_back(09);
    fila.push_back(11);
    fila.push_back(321);
    

    println!("{:?}", fila);

    println!("Tamanho da fila: {}", fila.len());


    //Verifica o último elemento da fila e sem remover
    if let Some(elemento) = fila.front () {
        println!("O que está na frente da fila {}", elemento);
    }

    //Remove elementos da fila
    println!("Removendo da lista");
    while let Some(elemento) = fila.pop_front() {
        println!("Removido {}", elemento);
    }

    //Verifica se a fila está vazia
    println!("A está vazia? {}", fila.is_empty());



}




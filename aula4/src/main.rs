fn main() {
    let x = 10; // `x` é o dono do valor 10
    let y = &x; // `y` é uma referência imutável para `x`
    println!("Valor de x: {}, via referência: {}", x, y);

    // Para modificar o valor, precisamos de uma referência mutável:
    let mut z = 20;
    {
        let w = &mut z; // Empréstimo mutável; só pode existir uma referência mutável ativa
        *w += 5; // Desreferenciamos `w` para alterar o valor
    }
    println!("Novo valor de z: {}", z);
}


fn main() {
    // Cria um Box que contém um valor inteiro.
    let b = Box::new(5);
    println!("Valor dentro do Box: {}", b);
    // Quando `b` sai de escopo, a memória é automaticamente liberada.
}



use std::rc::Rc;

fn main() {
    let rc_value = Rc::new(10);
    let rc_clone1 = Rc::clone(&rc_value);
    let rc_clone2 = Rc::clone(&rc_value);

    println!("Contagem de referências: {}", Rc::strong_count(&rc_value));
    // Saída: Contagem de referências: 3
}



use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    {
        let mut borrowed = data.borrow_mut(); // Empréstimo mutável em tempo de execução
        *borrowed += 1;
    }
    println!("Valor atualizado: {}", data.borrow());
}



fn main() {
    let mut num = 5;
    // Cria um ponteiro cru para num.
    let r1: *mut i32 = &mut num;

    // Bloco unsafe para desreferenciar o ponteiro.
    unsafe {
        *r1 += 1;
        println!("Valor modificado via ponteiro cru: {}", *r1);
    }
}

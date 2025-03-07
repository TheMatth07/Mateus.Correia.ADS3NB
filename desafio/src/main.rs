struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct Queue<T> {
    front: Option<Box<Node<T>>>,
    back: *mut Option<Box<Node<T>>>,
    length: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            front: None,
            back: std::ptr::null_mut(),
            length: 0,
        }
    }

    pub fn enqueue(&mut self, elem: T) {
        let new_node = Box::new(Node {
            value: elem,
            next: None,
        });

        if self.length == 0 {
            self.front = Some(new_node);
            self.back = &mut self.front;
        } else {
            unsafe {
                (*self.back).as_mut().unwrap().next = Some(new_node);
            }
            self.back = unsafe {
                &mut (*self.back).as_mut().unwrap().next
            };
        }
        self.length += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.front.take().map(|node| {
            self.front = node.next;
            if self.length == 1 {
                self.back = std::ptr::null_mut();
            }
            self.length -= 1;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.front.as_ref().map(|node| &node.value)
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        let mut current = self.front.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
    fn test_enqueue_dequeue() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.len(), 3);

        assert_eq!(queue.dequeue(), Some(10));
        assert_eq!(queue.dequeue(), Some(20));
        assert_eq!(queue.dequeue(), Some(30));
        assert_eq!(queue.dequeue(), None);

        assert_eq!(queue.len(), 0);
    }

    
    fn test_peek() {
        let mut queue: Queue<i32> = Queue::new();
        queue.enqueue(10);
        queue.enqueue(20);

        assert_eq!(queue.peek(), Some(&10));

        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_empty_queue() {
        let mut queue: Queue<i32> = Queue::new();
        
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.peek(), None);
    }
}

fn main() {
    // Criando uma instância da fila
    let mut queue: Queue<i32> = Queue::new();

    // Adicionando elementos à fila
    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);

    // Exibindo o primeiro elemento da fila (sem remover)
    match queue.peek() {
        Some(front) => println!("O primeiro da fila é: {}", front),
        None => println!("A fila está vazia."),
    }

    // Removendo elementos da fila e exibindo
    println!("Removendo elementos da fila:");
    while let Some(value) = queue.dequeue() {
        println!("Dequeue: {}", value);
    }

    // Verificando o tamanho da fila (que agora deve estar vazio)
    println!("Tamanho da fila após os deques: {}", queue.len());
}
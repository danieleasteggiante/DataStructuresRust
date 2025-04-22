use std::fmt::Debug;

#[derive(Debug)]
pub struct Node<T> {
    pub value: Option<T>,
    pub next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
    pub len: usize,
}

impl<T: PartialEq + Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None , len: 0 }
    }

    pub fn add(&mut self, value: T) {
        self.len += 1;
        let new_node = Node {
            value: Some(value),
            next: None,
        };
        let boxed_node = Box::new(new_node);
        if self.head.is_none() {
            self.head = Some(boxed_node);
            return;
        }
        let mut current = self.head.as_mut().unwrap();
        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }
        current.next = Some(boxed_node);
    }
    pub fn traverse(&self) {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            println!("{:?}", node);
            current = node.next.as_ref();
        }
    } 
    pub fn remove(&mut self, target: T) {
        let mut current = &mut self.head;
        let value = Some(target);
        // every unwrap function move the value, in case of generics this is a problem because are not clonable.
        while let Some(node) = current {
            // to compare without move from Option(Foo) you need to compare 2 Option. because if you try to unwrap the value 
            // from Option(value) for ex value.unwrap(), the value will be moved and you ll get a borrowing error.
            if node.value == value {
                *node = node.next.take().expect("Error method not present");
                self.len-=1;
                return;
            }
            current = &mut node.next;
        }
    }
    
    pub fn remove_by_index(&mut self, index: usize) {
        if index >= self.len {
            return;
        }
        let mut current = &mut self.head;
        for i in 0..=index {
            if i == index {
                *current = current.as_mut().unwrap().next.take();
                self.len-=1;
                return;
            }
            current = &mut current.as_mut().unwrap().next;
        }
    }
    pub fn reverse(&mut self) {
        let mut current = &mut self.head;
        let mut prev: Option<Box<Node<T>>> = None;
        let mut next: Option<Box<Node<T>>> = None;
        while let Some(mut node) = current.take() {
            next = node.next.take(); 
            node.next = prev.take(); 
            if next.is_none() {
                self.head = Some(node); 
                return;
            }
            prev = Some(node); 
            current = &mut next; 
        }
    }

}
use std::ops::Deref;

// use std::iter::Iterator;

#[derive(Debug)]
pub struct MyList<T>{
    head: Option<Node<T>>
}
#[derive(Debug)]
struct Node<T>{
    value: T,
    next: Option<Box<Node<T>>>
}

pub struct MyListIterator<'a, T>{
    next: Option<&'a Node<T>>
}


impl<T> Node<T> {
    fn new(val: T) ->Self{
        Node {
            value: val,
            next: None
        }
    }

    fn push(&mut self, val: T){
        let mut temp = self;
        while !temp.next.is_none() {
            temp = temp.next.as_mut().unwrap()
        }
        temp.next = Some(Box::new(Node::new(val)));
    }

    fn pop(&mut self) -> Option<T>{
        let mut temp = self;
        while !temp.next.as_ref().unwrap().next.is_none(){
            temp = temp.next.as_mut().unwrap();
        }
        let replacement = temp.next.take();
        Some(replacement.unwrap().value)
    }

    fn remove(&mut self) -> T{
        let replacement;
        let mut temp = self;
        while !temp.next.as_ref().unwrap().next.is_none(){
            temp = temp.next.as_mut().unwrap();
        }
        replacement = temp.next.take();
        replacement.unwrap().value
    }

    fn peek(&self)->&T{
        &self.value
    }
}

impl<T> MyList<T>{
    pub fn new(val: T) -> Self{
        MyList{
            head: Some(Node::new(val))
        }
    }

    pub fn push(&mut self,val: T){
        self.head.as_mut().unwrap().push(val);
    }

    pub fn pop(&mut self) -> Option<T>{
        if self.head.is_none(){return None;}
        else if self.head.as_ref().unwrap().next.is_none(){
            let replacement = self.head.take();
            return Some(replacement.unwrap().value);
        }else{
            self.head.as_mut().unwrap().pop()
        }
    }

    pub fn peek_beginning(&self) -> Option<&T>{
        self.peek_at(0)
    }

    pub fn peek_at(&self,at:usize) -> Option<&T>{
        let mut temp = self.iter().skip(at);
        temp.next()
    }

    pub fn remove(&mut self)-> T{
        if self.head.is_none() {
            panic!("Empty lol")
        }
        if self.head.as_ref().unwrap().next.is_none(){
            let replacement = self.head.take();
            return replacement.unwrap().value;
        }
        self.head.as_mut().unwrap().remove()
    }
    
    pub fn iter<'a>(&'a self)->MyListIterator<'a,T>{
        let iterable = self.head.as_ref();
        MyListIterator{
            next: iterable
        }
    }

    pub fn _index(&mut self, _position: u32){
        unimplemented!();
    }
}

impl<'a, T> Iterator for MyListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self)->Option<Self::Item>{
        if self.next.is_none(){
            return None;
        }
        let temp = self.next.as_ref().unwrap().peek();
        if self.next.as_ref().unwrap().next.is_some()
        {
            let next_next = self.next.as_ref().unwrap().next.as_ref().unwrap().deref();
            self.next = Some(next_next);
        }else{
            self.next = None;
        }
        Some(temp)
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn popper(){
        let mut ll = MyList::new(6);
        ll.push(7);
        ll.push(8);
        ll.pop();
        let mut iter_ll = ll.iter();
        assert_eq!(iter_ll.next(),Some(&6));
        assert_eq!(iter_ll.next(),Some(&7));
        assert_eq!(iter_ll.next(),None);
    }
}
use core::panic;
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

pub struct MyListIterMut<'a, T>{
    next: Option<&'a mut Node<T>>
}

pub struct MyListIntoIter<T>(MyList<T>);

#[allow(dead_code)]
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

    fn insert_node(&mut self, val:T){
        let mut new_node = Self::new(val);
        new_node.next = self.next.take();
        self.next = Some(Box::new(new_node))
    }

    fn push_at(&mut self, position: usize, val: T){
        let mut temp = self;
        let mut i = 0 as usize;
        loop {
            if i == position - 1{
                break;
            }
            temp = temp.next.as_deref_mut().unwrap();
            i+=1;
        }
        temp.insert_node(val)
    }
}

#[allow(dead_code)]
impl<T> MyList<T>{
    pub fn new(val: T) -> Self{
        MyList{
            head: Some(Node::new(val))
        }
    }

    pub fn push(&mut self,val: T){
        if self.is_empty(){
            *self = MyList::new(val);
            return;
        }
        self.head.as_mut().unwrap().push(val);
    }

    pub fn pop(&mut self) -> Option<T>{
        if self.is_empty(){return None;}
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
        if self.is_empty() {
            panic!("Can't remove since list is empty.")
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

    pub fn push_front(&mut self, val:T){
        if self.is_empty(){
            *self = MyList::new(val);
            return;
        }
        let mut new_head = Node::new(val);
        let new_next = self.head.take();
        new_head.next = Some(Box::new(new_next.unwrap()));
        self.head = Some(new_head);
    }
    
    pub fn pop_front(&mut self)-> Option<T>{
        if self.is_empty(){
            return None;
        }
        let mut return_val = self.head.take();
        let old_head_next = return_val.as_mut().unwrap().next.take();
        if old_head_next.is_some()
        {
            self.head = Some(*(old_head_next.unwrap()));
        }else{
            self.head = None;
        }

        Some(return_val.unwrap().value)
    }

    pub fn is_empty(&self) -> bool{
        self.head.is_none()
    }

    pub fn into_iter(self) -> MyListIntoIter<T>{
        MyListIntoIter(self)
    }

    pub fn iter_mut<'a>(&'a mut self) -> MyListIterMut<'a, T>{
        MyListIterMut{
            next: self.head.as_mut()
        }
    }

    pub fn push_at(&mut self, position: usize, val: T){
        if position == 0{
            self.push_front(val);
            return;
        }
        if self.peek_at(position).is_none(){
            if self.peek_at(position-1).is_some(){
                self.push(val);
                return;
            }
            else{
                panic!("Can't push over the limit 🤷‍♀️")
            }
        }
        self.head.as_mut().unwrap().push_at(position,val)
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

impl<T> Iterator for MyListIntoIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>{
        self.0.pop_front()
    }
}

impl<'a,T> Iterator for MyListIterMut<'a,T>{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item>{
        if self.next.is_none(){
            return None;
        }
        self.next.take().map(|node | {
            self.next = node.next.as_deref_mut();
            &mut node.value
        }
        )
        
    }
}


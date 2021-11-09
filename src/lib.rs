//! MyList
//! 
//! `MyList` is my attempt to implement a Linked List in Rust.


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
    /// Creates a new empty Linked List
    /// 
    /// # Examples
    /// ```
    /// use daa::lib::MyList;
    /// let list: MyList<u32> = MyList::new_null();
    /// ```
    /// 
    pub fn new_null()->Self{
        MyList{
            head: None
        }
    }

    /// Creates a new linked list but with a value.
    /// 
    /// # Examples
    /// ```
    /// use daa::lib::MyList;
    /// let list: MyList<u32> = MyList::new(5);
    /// ```
    /// 
    pub fn new(val: T) -> Self{
        MyList{
            head: Some(Node::new(val))
        }
    }

    /// Inserts element at the end of the Linked List
    /// 
    /// # Examples
    /// ```
    /// use daa::lib::MyList;
    /// let mut list: MyList<u32> = MyList::new(4);
    /// list.push(5);
    /// assert_eq!(list.peek(1),Some(&5));
    /// ```
    /// 
    pub fn push(&mut self,val: T){
        if self.is_empty(){
            *self = MyList::new(val);
            return;
        }
        self.head.as_mut().unwrap().push(val);
    }

    /// Removes element at the end of the Linked List and wraps it in the Option Enum
    /// Return None if List is empty.
    /// 
    /// # Examples
    /// ```
    /// use daa::lib::MyList;
    /// let mut list: MyList<u32> = MyList::new(4);
    /// list.push(5);
    /// let popped_element: Option<u32> = list.pop();
    /// let another_popped_element: Option<u32> = list.pop();
    /// let none_value: Option<u32> = list.pop(); // this will return None since List is empty
    /// assert_eq!(popped_value,Some(5));
    /// assert_eq!(another_popped_value, Some(4));
    /// assert_eq!(none_value, None);
    /// ```
    /// 
    pub fn pop(&mut self) -> Option<T>{
        if self.is_empty(){return None;}
        else if self.head.as_ref().unwrap().next.is_none(){
            let replacement = self.head.take();
            return Some(replacement.unwrap().value);
        }else{
            self.head.as_mut().unwrap().pop()
        }
    }

    /// Returns  a reference to the element at the beginning of the list wrapped in an option enum
    /// 
    /// # Examples
    /// ```
    /// use daa::lib::MyList;
    /// let list: MyList<u32> = MyList::new(4);
    /// let first_value_peek: Option<&u32> = list.peek_beginning();
    /// assert_eq!(first_value_peek,Some(&4));
    /// ```
    /// 
    pub fn peek_beginning(&self) -> Option<&T>{
        self.peek_at(0)
    }

    /// Returns reference to an element at a particular position on the List wrapped in the Option enum.
    /// Returns None if List is empty;
    /// 
    /// # Examples
    /// ```
    /// use daa::lib::MyList;
    /// let mut list: MyList<u32> = MyList::new(4);
    /// list.push(5);
    /// list.push(6);
    /// // if i want to see the 3rd (position will be 2) value in the list;
    /// let peek_at_0: Option<&u32> = list.peek_at(0);
    /// let peek_at_1: Option<&u32> = list.peek_at(1);
    /// let peek_at_2: Option<&u32> = list.peek_at(2); 
    /// assert_eq!(peek_at_0,Some(&4));
    /// assert_eq!(peek_at_1,Some(&5));
    /// assert_eq!(peek_at_2,Some(&6));
    /// ```
    /// 
    pub fn peek_at(&self,at:usize) -> Option<&T>{
        let mut temp = self.iter().skip(at);
        temp.next()
    }

    /// Removes an element from the end of the list. The element is not wrapped in the Option Enum.
    /// Panics if list is empty
    /// However I do not recommend using this. Use pop or pop front for safety 🤷‍♂️
    /// I didn't even bother to make a remove_front because this shouldn't be used.
    /// # Examples
    /// ```
    /// use daa::lib::MyList;
    /// let mut list: MyList<u32> = MyList::new(4);
    /// list.push(5);
    /// list.push(6);
    /// let removed_value: u32 = list.remove();
    /// assert_eq!(removed_value, 6)
    /// ```
    /// 
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
    
    /// Provides a forward iterator.
    /// Loops through all the elements in the list and returns a reference to them.
    /// 
    /// # Examples
    /// ```
    /// use daa::lib::MyList;
    /// let mut list: MyList<u32> = MyList::new(4);
    /// list.push(5);
    /// list.push(6);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(),Some(&4));
    /// assert_eq!(iter.next(),Some(&5));
    /// assert_eq!(iter.next(),Some(&6));
    /// assert_eq!(iter.next(),None)
    /// ```
    /// 
    pub fn iter<'a>(&'a self)->MyListIterator<'a,T>{
        let iterable = self.head.as_ref();
        MyListIterator{
            next: iterable
        }
    }

    /// Inserts element to the front of the Linked List
    /// 
    /// # Examples
    /// ```
    /// use daa::lib::MyList;
    /// let mut list: MyList<u32> = MyList::new(4);
    /// list.push(5);
    /// list.push_front(6); //pushes 6 to the front
    /// let element_at_the_front: Option<&u32> = list.peek_beginning(); 
    /// assert_eq!(element_at_the_front,Some(&6));
    /// ```
    /// 
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
    
    /// Removes element from the the front of the Linked List and wraps it in an option value
    /// 
    /// # Examples
    /// ```
    /// use daa::lib::MyList;
    /// let mut list: MyList<u32> = MyList::new(4);
    /// list.push(5);
    /// list.push(6);
    /// let popped_value = list.pop_front(); // we just removed 4 from the front
    /// assert_eq!(popped_value, Some(4));
    /// ```
    /// 
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

    /// Checks if list is empty and returns a boolean
    /// 
    /// # Examples
    /// ```
    /// use daa::lib::MyList;
    /// let list: MyList<u32> = MyList::new(4);
    /// let empty_list: MyList<u32> = MyList::new_null();
    /// assert!(empty_list.is_empty())
    /// assert!(!list.is_empty())
    /// ```
    /// 
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

    /// Inserts an element a specific location. Panics if it goes beyond length+1
    /// 
    /// # Examples
    /// ```
    /// use daa::lib::MyList;
    /// let mut list: MyList<u32> = MyList::new(4);
    /// list.push(5); // 5 is at position 1 now
    /// list.push(6);
    /// list.push_at(1, 7); // now 7 is at position 1
    /// let element_at_1 = list.peek_at(1);
    /// asserteq!(element_at_1, Some(&7));
    /// ```
    /// 
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


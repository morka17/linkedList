use core::fmt;
use core::iter::{FromIterator, FusedIterator};
use core::marker::PhantomData;
use core::mem;
/// A doubly-linked list
///
/// The `LinkedList` allows pushing and poping elements at eitehr end
/// in constant time
///
/// use LinkedList::
///
/// let list = LinkedList::from([1,2,3]);
///
use core::ptr::NonNull;

use std::boxed::Box; // Allocates memory on the heap

#[derive(Clone, Debug)]
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    /// Adds the given node to the front of the list.
    #[inline]
    fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        // This method takes care not to create mutable reference to whole nodes,
        unsafe {
            node.next = self.head;
            node.prev = None;
            let node = Some(Box::leak(node).into());

            match self.head {
                None => self.tail = node,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(head) => (*head.as_ptr()).prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    #[inline]
    fn push_back_node(&mut self, mut node: Box<Node<T>>){
        unsafe {
            node.next = None;
            node.prev = self.tail;

            let node = Some(Box::leak(node).into());

            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = node,
            }

            self.tail = node;
            self.len += 1;
            
        }
    }

    /// Removes and returns the node at the front of the list.
    #[inline]
    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            match self.head {
                None => self.tail = None,
                // Not creating new mutable (unique!) references overlapping `element`
                Some(head) => (*head.as_ptr()).prev = None,
            }

            self.len -= 1;
            node
        })
    }

    #[inline]
    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;

            match self.tail {
                None => self.head = None,
                // Not creating new mutable (unique!) references overlapping `element`
                Some(tail) => (*tail.as_ptr()).prev = None,
            }

            self.len -= 1;
            node
        })
    }
    
}


impl<T> Default for LinkedList<T>{
    /// Creates an empty `LinkedList<T>`
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}


impl<T> LinkedList<T>{
    pub const fn new() -> Self {
        LinkedList {head: None, tail: None, len: 0, marker: PhantomData}
    }

    pub fn append(&mut self, other: &mut Self){
        match self.tail{
            None => mem::swap(self, other),
            Some(mut tail) => {
                if let Some(mut other_head) = other.head.take(){
                    unsafe {
                        tail.as_mut().next = Some(other_head);
                        other_head.as_mut().prev = Some(tail);
                    }

                    self.tail = other.tail.take();
                    self.len += mem::replace(&mut other.len, 0);
                }
            }
        }
    }

    #[inline]
    pub fn clear(&mut self){
        *self = Self::new();
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn contains(&self, x: &T) -> bool 
    where
        T: PartialEq<T>,
    {
      //TODO: Fix here
      //  self.iter().any(|e| e == x)
      true
    }

    pub fn front(&self) -> Option<&T>{
        unsafe{self.head.as_ref().map(|node | &node.as_ref().element)}
    }

    pub fn back(&self) -> Option<&T>{
        unsafe{self.tail.as_ref().map(|node | &node.as_ref().element)}
    }

    pub fn push_front(&mut self, elt: T) {
        self.push_front_node(Box::new(Node::new(elt)));
    }

    pub fn push_back(&mut self, elt: T) {
        self.push_back_node(Box::new(Node::new(elt)));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(Node::into_element)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(Node::into_element)
    }
}

struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    element: T,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node {
            next: None,
            prev: None,
            element,
        }
    }
    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}

/// An iterator over the elements of a `LinkedList`
pub struct Iter<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<T: fmt::Debug> fmt::Debug for Iter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Iter")
            // .field(&*mem::ManuallyDrop::new(LinkedList{
            //     head: self.head,
            //     tail: self.tail,
            //     len: self.len,
            //     marker: PhantomData,
            // }))
            .field(&self.len)
            .finish()
    }
}

impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter { ..*self }
    }
}

/// A mutable iterator over the elements of a `LinkedList`
pub struct IterMut<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<T: fmt::Debug> fmt::Debug for IterMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IterMut")
            .field(&*mem::ManuallyDrop::new(LinkedList {
                head: self.head,
                tail: self.tail,
                len: self.len,
                marker: PhantomData,
            }))
            .field(&self.len)
            .finish()
    }
}

#[derive(Clone)]
pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.list).finish()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn push_back_it_work(){
        let mut d = LinkedList::new();

        d.push_back(1);

        assert_eq!(d.back().unwrap(), &1);
    }

    #[test]
    fn push_front_it_work(){
        let mut d = LinkedList::new();

        d.push_front(1);

        assert_eq!(d.front().unwrap(), &1);
    }

    #[test]
    fn pop_front_it_work(){
        let mut d = LinkedList::new();

        d.push_front(1);

        assert_eq!(d.pop_front(), Some(1));
        assert_eq!(d.pop_front(), None);
    }

    #[test]
    fn pop_back_it_work(){
        let mut d = LinkedList::new();

        d.push_front(1);
        d.push_front(3);

        assert_eq!(d.pop_back(), Some(3));
       assert_eq!(d.pop_back(), None);
    }

    
}
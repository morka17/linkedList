use std::ptr::NonNull;
use std::marker::PhantomData;

struct Node<T>{
    element: T, 
    next: Option<NonNull<Node<T>>>
}

impl<T> Node<T>{
    fn new(element: T) -> Self{
        Node {next: None, element}
    }

    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}

#[derive(Clone, Debug)]
pub struct SinglyLinkedList<T>{
    head: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>
}


// private methods
impl<T> SinglyLinkedList<T>{
    /// Adds the given node to the front of the list.__rust_force_expr!
    #[inline]
    fn add_front_node(&mut self, mut node: Box<Node<T>>){
        // This method takes care not to create mutable references to whole nodes,
        //  to maintain validity of aliasing pointers into `element`.
        unsafe {
            node.next = self.head;
            let node = Some(Box::leak(node).into());

            match self.head{
                None => self.head = node,
                Some(head) => (*head.as_ptr()).next = node,
            }

            self.head = node;
            self.len += 1;
        }
    }
    
    /// Removes and returns the node at the front of the list.
    #[inline]
    fn remove_front_node(&mut self) -> Option<Box<Node<T>>>{
        // This method tales care not to create mutable reference to whole nodes,
        // tp maintain validity of aliasing pointers into `element.`
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            // match self.head {
            //     None => self.head = None,
            //     Some(head) => (*head.as_ptr()).next = N
            // }

            self.len -= 1;
            node
          
        })
    }
}



// User crate interface 
impl<T> SinglyLinkedList<T>{
    /// Creates an empty `SingleLinkedList`
    /// 
    /// # Examples
    /// 
    /// ```
    /// let list: SingleLinkedList<u32> = SingleLinkedList::new();
    /// 
    /// ```
    #[inline]
    pub const fn new() -> Self{
        SinglyLinkedList{head:None, len: 0, marker: PhantomData}
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize{
        self.len
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    #[inline]
    #[must_use]
    pub fn front(&self) -> Option<&T>{
        unsafe{
            self.head.as_ref().map(|node| &node.as_ref().element)
        }
    }

    
    pub fn add_front(&mut self, elt: T) {
        self.add_front_node(Box::new(Node::new(elt)));
    }

    pub fn remove_front(&mut self) -> Option<T>{
        self.remove_front_node().map(Node::into_element)
    }

} 




#[cfg(test)]
mod test{
    use super::*;
    // #[test]
    // fn front_it_works() {
        
    // }

    fn add_it_works() {
        let mut list: SinglyLinkedList<u32> = SinglyLinkedList::new();

        list.add_front(10);
        list.add_front(20);
        list.add_front(30);

        assert_eq!(list.is_empty(), false)

    }
}
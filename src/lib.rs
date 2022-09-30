


mod single_linked_list;
mod doubly_linked_list;

pub use doubly_linked_list::LinkedList;
pub use single_linked_list::SinglyLinkedList;




pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

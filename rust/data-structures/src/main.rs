use crate::sll::SinglyLinkedList;

mod sll;

fn main() {
    let mut list = SinglyLinkedList::new(1);
    list.append(2);
    list.append(3);
    let mut item = list.head();
    loop {
        println!("item: {}", item.data);
        if let Some(next) = item.next() {
            item = next;
        } else {
            break;
        }
    }
}

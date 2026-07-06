use ds::list::LinkedList;

fn main() {
    let mut list = LinkedList::new();

    list.insert_at_head(1);
    list.insert_at_head(2);
    list.insert_at_head(3);

    println!("{}", list);
    println!("{}", list.get(1).unwrap_or(&-1));
}

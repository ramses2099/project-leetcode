use std::collections::LinkedList;

type List = LinkedList<i32>;

fn add_two_numbers(l1: List, l2: List) -> List {
    let mut l1 = l1;
    let mut iter = l1.iter_mut();
    // for x in iter {
    //     print!("{:?}", *x);
    // }
    while let Some(x) = iter.next() {
        println!("{}", *x);
    }
    l2
}

fn main() {
    let mut list1: LinkedList<i32> = LinkedList::from([2,4,3]);
    println!("List 1: {:?}", list1);

    let mut list2: LinkedList<i32> = LinkedList::from([5,6,4]);
    println!("List 2: {:?}", list2);

    let mut iter1 = list1.iter().rev();
    let mut iter2 = list2.iter().rev();
    let mut result: LinkedList<i32> = LinkedList::new();

    while let (Some(x), Some(y)) = (iter1.next(), iter2.next()) {
        let sum = *x + *y;
        result.push_back(sum);
    }
    println!("Result: {:?}", result);
}

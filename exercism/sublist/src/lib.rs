#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sublist_of<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.is_empty() {
        return true;
    }
    second_list
        .windows(first_list.len())
        .any(|window| window == first_list)
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use Comparison::{Equal, Sublist, Superlist, Unequal};

    if first_list == second_list {
        Equal
    } else if sublist_of(first_list, second_list) {
        Sublist
    } else if sublist_of(second_list, first_list) {
        Superlist
    } else {
        Unequal
    }
}

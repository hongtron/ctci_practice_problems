use std::collections::HashMap;
use std::collections::LinkedList;

fn remove_dups<T: Eq + std::hash::Hash + Copy>(l: LinkedList<T>) -> LinkedList<T> {
    let mut deduped: LinkedList<T> = LinkedList::new();
    let mut elements: HashMap<T, bool> = HashMap::new();
    l.iter().for_each(|e|
        if elements.get(e).is_none() {
            deduped.push_back(*e);
            elements.insert(*e, true);
        });
        deduped
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_dups() {
        let mut list_with_dups: LinkedList<u32> = LinkedList::new();
        list_with_dups.push_front(8);
        list_with_dups.push_front(6);
        list_with_dups.push_front(7);
        list_with_dups.push_front(7);
        list_with_dups.push_front(9);

        let mut deduped_list: LinkedList<u32> = LinkedList::new();
        deduped_list.push_front(8);
        deduped_list.push_front(6);
        deduped_list.push_front(7);
        deduped_list.push_front(9);
        assert_eq!(remove_dups(list_with_dups), deduped_list)
    }
}

pub fn binary_search(list: &Vec<usize>, needle: usize) -> bool {
    let mut low = 0;
    let mut high = list.len();

    while low < high {
        let middle = (low + (high - low) / 2).into();

        let result = list[middle];

        if result == needle {
            return true;
        } else if needle > result {
            low = middle + 1;
        } else {
            high = middle;
        }
    }

    false
}

#[cfg(test)]
mod tests {

    use super::binary_search;

    #[test]
    fn binary_search_array() {
        let foo = vec![1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];

        assert!(binary_search(&foo, 69));
        assert!(!binary_search(&foo, 1336));
        assert!(binary_search(&foo, 69420));
        assert!(!binary_search(&foo, 69421));
        assert!(binary_search(&foo, 1));
        assert!(!binary_search(&foo, 0));
    }
}

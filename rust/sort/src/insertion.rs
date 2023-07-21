pub fn insertion_sort<T: Ord + Clone>(list: &mut Vec<T>) {
    for i in 1..list.len() {
        let key = list[i].clone();

        let mut j = i;

        while j > 0 && key < list[j - 1] {
            list.swap(j, j - 1);

            j -= 1;
        }

        list[j] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;

    #[test]
    fn insertion_sort_test() {
        let mut list = vec![8, 2, 5, 1, 6, 4, 3];

        insertion_sort(&mut list);

        assert_eq!(vec![1, 2, 3, 4, 5, 6, 8], list);
    }
}

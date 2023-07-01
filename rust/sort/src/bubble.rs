pub fn bubble_sort<T: PartialEq + PartialOrd>(list: &mut Vec<T>) {
    for i in 0..list.len() {
        for j in 0..list.len() - 1 - i {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn bubble_sort_test() {
        let mut list = vec![8, 2, 5, 1, 6, 4, 3];

        bubble_sort(&mut list);

        assert_eq!(vec![1, 2, 3, 4, 5, 6, 8], list);
    }
}

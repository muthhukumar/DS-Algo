pub fn search(list: &Vec<usize>, needle: usize) -> bool {
    let mut low = 0;
    let mut high = list.len();

    while low < high {
        let middle = (low + (high - low) / 2).into();

        let result = list[middle];

        if result == needle {
            return true;
        } else if result > middle {
            low = middle + 1;
        } else {
            high = middle;
        }
    }

    false
}

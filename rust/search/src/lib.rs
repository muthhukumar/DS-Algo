pub fn binary_search(list: &Vec<usize>, needle: usize) -> bool {
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

pub fn linear_search<T: PartialEq>(list: &Vec<T>, needle: T) -> bool {
    for item in list {
        if *item == needle {
            return true;
        }
    }

    false
}

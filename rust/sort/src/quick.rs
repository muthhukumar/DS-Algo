pub fn partition(arr: &mut Vec<usize>, low: usize, high: usize) -> usize {
    let pivot = arr[high];

    let mut idx = low;

    for i in low..high {
        if arr[i] <= pivot {
            arr.swap(i, idx);
            idx += 1;
        }
    }

    arr.swap(high, idx);
    arr[idx] = pivot;

    idx
}

pub fn qs(arr: &mut Vec<usize>, low: usize, high: usize) {
    if low >= high {
        return;
    }

    let pivot_idx = partition(arr, low, high);

    if let Some(pivot) = pivot_idx.checked_sub(1) {
        qs(arr, low, pivot);
    }

    if let Some(pivot) = pivot_idx.checked_add(1) {
        qs(arr, pivot, high);
    }
}

#[cfg(test)]
mod tests {
    use super::qs;

    #[test]
    fn quick_sort() {
        let mut arr = vec![8, 1, 7, 3, 5, 2];

        let len: usize = arr.len() - 1;

        qs(&mut arr, 0, len);

        assert_eq!(vec![1, 2, 3, 5, 7, 8], arr);
    }
}

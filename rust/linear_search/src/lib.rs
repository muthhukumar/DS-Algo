pub fn search<T: PartialEq>(list: &Vec<T>, needle: T) -> bool {
    for item in list {
        if *item == needle {
            return true;
        }
    }

    false
}

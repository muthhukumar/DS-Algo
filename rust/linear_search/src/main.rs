fn main() {
    let list = vec![1, 23, 3, 4, 5, 6, 7, 7, 9];

    let result = linear_search::search(&list, 23);

    println!("result: {result}");
}

fn fizzbuzz() {
    ["fizz"].into_iter()
        .chain(["";2])
        .cycle()
        .map(String::from)
        .zip(
            ["buzz"].into_iter()
                .chain(["";4])
                .cycle()
        )
        .map(|(fizz, buzz)| fizz + buzz)
        .enumerate()
        .map(|(idx, string)| 
            string.is_empty()
                .then_some(idx.to_string())
                .unwrap_or(string)
        )
        .skip(1)
        .take(100)
        .for_each(|string| println!("{string}"))
}
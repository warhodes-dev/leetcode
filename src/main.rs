fn main() {
    ["fizz"].into_iter()
        .chain(["";2])
        .cycle()
        .zip(
            ["buzz"].into_iter()
                .chain(["";4])
                .cycle()
        )
        .map(|(fizz, buzz)| format!("{fizz}{buzz}"))
        .enumerate()
        .map(|(idx, string)| {
            string.is_empty()
                .then_some(idx.to_string())
                .unwrap_or(string)
        })
        .skip(1)
        .take(100)
        .for_each(|string| println!("{string}"));
}

fn verbose() {
    let fizzes = ["fizz"]
        .into_iter()
        .chain(["";2])
        .cycle();

    let buzzes = ["buzz"]
        .into_iter()
        .chain(["";4])
        .cycle();

    let fizzbuzzes = fizzes.zip(buzzes)
        .map(|(fizz, buzz)| format!("{fizz}{buzz}"));

    let fizzbuzz = fizzbuzzes
        .enumerate()
        .map(|(idx, string)| {
            string.is_empty()
                .then_some(idx.to_string())
                .unwrap_or(string)
        });

    fizzbuzz
        .skip(1)
        .take(100)
        .for_each(|string| println!("{string}"));
}
pub fn run(source: String) {
    let tokens: Vec<&str> = source.split_whitespace().collect();
    for token in tokens {
        println!("{}", token);
    }
}

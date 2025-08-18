pub mod token;

pub fn run(source: String) -> anyhow::Result<()> {
    let tokens: Vec<&str> = source.split_whitespace().collect();
    for token in tokens {
        println!("{}", token);
    }

    Ok(())
}

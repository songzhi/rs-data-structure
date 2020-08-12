use std::io::{BufRead, Write};

fn solution(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    let cost: usize = buf.trim().parse().unwrap();
    let mut change = 1024 - cost;
    let mut coins_count = 0;
    let coins = [64, 16, 4, 1];
    for &coin in coins.iter() {
        coins_count += change / coin;
        change %= coin;
    }
    writeln!(output, "{}", coins_count);
}


fn main() {
    solution(std::io::stdin().lock(), std::io::stdout());
}

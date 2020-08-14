use std::io::{BufRead, Write};
use std::str::FromStr;


fn min_energy(buildings: impl Iterator<Item = f64>) -> usize {
    let mut e = 0.0;
    for b in buildings {
        e = (e + b) / 2.0;
    }
    e.ceil() as usize
}

fn solve(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    buf.clear();
    input.read_line(&mut buf);
    writeln!(
        output,
        "{}",
        min_energy(buf.trim_end().split(' ').rev().map(f64::from_str).flatten())
    );
}

fn main() {
    solve(std::io::stdin().lock(), std::io::stdout());
}

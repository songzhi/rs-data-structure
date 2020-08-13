use std::cmp::Reverse;
use std::io::{BufRead, Write};
use std::str::FromStr;

fn solve(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    let n: usize = buf.trim().parse().unwrap();
    let mut points = vec![];
    for _ in 0..n {
        buf.clear();
        input.read_line(&mut buf);
        let mut nums = buf.trim_end().split(' ').map(usize::from_str).flatten();
        points.push((nums.next().unwrap(), nums.next().unwrap()));
    }
    points.sort_by_key(|(_, y)| Reverse(*y));
    let mut max = 0;
    for (x, y) in points {
        if x > max {
            max = x;
            writeln!(output, "{} {}", x, y);
        }
    }
}

fn main() {
    solve(std::io::stdin().lock(), std::io::stdout());
}
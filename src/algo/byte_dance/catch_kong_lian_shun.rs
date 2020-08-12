use std::io::{BufRead, Write};

#[inline]
fn c(n: usize) -> usize {
    n.saturating_sub(1) * n / 2
}

fn solution(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    let mut nums = buf.trim().split(' ').map(str::parse::<usize>).flatten();
    let (n, d) = (nums.next().unwrap(), nums.next().unwrap());
    buf.clear();
    input.read_line(&mut buf);
    let buildings: Vec<usize> = buf
        .trim()
        .split(' ')
        .map(str::parse::<usize>)
        .flatten()
        .collect();
    let mut begin = 0;
    let mut end = 0;
    let mut count = 0;
    while end < n {
        while end >= 2 && (buildings[end] - buildings[begin] > d) {
            begin += 1;
        }
        count += c(end - begin);
        end += 1;
    }
    writeln!(output, "{}", count % 99997867);
}

fn main() {
    solution(std::io::stdin().lock(), std::io::stdout());
}

#[test]
fn test() {
    crate::test_it(|x,y|solution(x,y), "4 3\n1 2 3 4", "4\n");
}
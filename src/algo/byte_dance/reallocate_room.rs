use std::fmt::Display;
use std::io::{BufRead, Write};
use std::str::FromStr;

fn next_num(cur: usize, change: isize, n: usize) -> usize {
    let ans = cur as isize + change;
    if ans < 0 {
        n - 1
    } else if ans >= n as isize {
        0
    } else {
        ans as usize
    }
}

fn join<T: std::fmt::Display>(a: &[T]) -> String {
    use std::fmt::Write;
    let mut s = a.iter().fold(String::new(), |mut s, n| {
        write!(s, "{} ", n).ok();
        s
    });
    s.truncate(s.len() - 1);
    s
}

fn solve(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    let mut nums = buf.trim().split(' ').map(usize::from_str).flatten();
    let (n, x) = (nums.next().unwrap(), nums.next().unwrap());
    let x = x - 1;
    buf.clear();
    input.read_line(&mut buf);
    let mut nums = buf
        .trim()
        .split(' ')
        .map(usize::from_str)
        .flatten()
        .collect::<Vec<_>>();
    let (i, &p) = nums.iter().enumerate().rev().min_by_key(|s| s.1).unwrap();
    let mut j = next_num(i, 1, n);
    let mut count = 1;
    while !(nums[i] == 0 && j == x) {
        count += 1;
        nums[j] -= 1;
        j = next_num(j, 1, n);
    }
    nums[x] -= 1;
    nums[i] += count;
    writeln!(output, "{}", join(nums.as_slice()));
}

fn main() {
    solve(std::io::stdin().lock(), std::io::stdout());
}

#[test]
fn test() {
    crate::test_it(
        |x, y| solve(x, y),
        r#"3 1
6 5 1"#,
        r#"4 4 4
"#,
    )
}

use std::io::{BufRead, Write};
use std::str::FromStr;

#[inline]
fn get_abc(i: usize, a: isize, d1: isize, d2: isize) -> (isize, isize) {
    match i {
        0 => (a + d1, a + d1 + d2),
        1 => (a + d1, a + d1 - d2),
        2 => (a - d1, a - d1 + d2),
        3 => (a - d1, a - d1 - d2),
        _ => unreachable!(),
    }
}

fn could_tie(n: isize, k: isize, d1: isize, d2: isize) -> bool {
    if n % 3 != 0 {
        return false;
    }
    let x = [
        k - 2 * d1 - d2,
        k - 2 * d1 + d2,
        k + 2 * d1 - d2,
        k + 2 * d1 + d2,
    ];
    for i in 0..4 {
        let mut sum = 0;
        let mut t = x[i];
        while t > 0 {
            sum += t % 10;
            t /= 10;
        }
        if sum % 3 == 0 && x[i] / 3 >= 0 && x[i] / 3 <= k && x[i] <= n {
            let a = x[i] / 3;
            let (b, c) = get_abc(i, a, d1, d2);
            if b >= 0 && b <= k.min(n / 3) && c >= 0 && c <= k.min(n / 3) {
                return true;
            }
        }
    }
    false
}

fn solve(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    let n = buf.trim().parse::<usize>().unwrap();
    for _ in 0..n {
        buf.clear();
        input.read_line(&mut buf);
        let mut nums = buf.trim().split(' ').map(isize::from_str).flatten();
        if could_tie(
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
        ) {
            writeln!(output, "yes");
        } else {
            writeln!(output, "no");
        }
    }
}

fn main() {
    solve(std::io::stdin().lock(), std::io::stdout());
}

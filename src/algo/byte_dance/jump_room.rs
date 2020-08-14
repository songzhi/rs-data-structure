use std::io::{BufRead, Write};
use std::str::FromStr;

const MOD: isize = 1e9 as isize + 7;

fn solve(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    buf.clear();
    input.read_line(&mut buf);
    let p: Vec<_> = buf
        .trim_end()
        .split(' ')
        .map(usize::from_str)
        .flatten()
        .collect();
    let n = p.len();
    let mut dp = vec![0; n + 2];
    for i in 2..=n + 1 {
        dp[i] = (2 * dp[i - 1] - dp[p[i - 2]] + 2) % MOD;
    }
    writeln!(
        output,
        "{}",
        if dp[n + 1] < 0 {
            dp[n + 1] + MOD
        } else {
            dp[n + 1]
        }
    );
}

fn main() {
    solve(std::io::stdin().lock(), std::io::stdout());
}

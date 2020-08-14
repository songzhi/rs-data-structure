use std::io::{BufRead, Write};
use std::str::FromStr;

fn solve(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    buf.clear();
    input.read_line(&mut buf);
    let mut nums: Vec<usize> = buf
        .trim_end()
        .split(' ')
        .map(usize::from_str)
        .flatten()
        .collect();
    nums.sort_unstable();
    let nums_count = nums.len();
    let mut ans = 0;
    let first = nums[0];
    nums.into_iter().fold(first, |prev, cur| {
        if cur - prev > 10 {
            ans += 1;
        }
        cur
    });
    if (nums_count + ans) % 3 != 0 {
        ans = ans + 3 - (nums_count + ans) % 3;
    }
    writeln!(output, "{}", ans);
}

fn main() {
    solve(std::io::stdin().lock(), std::io::stdout());
}

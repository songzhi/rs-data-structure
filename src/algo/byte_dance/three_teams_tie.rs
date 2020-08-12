use std::cmp::max;
use std::io::{BufRead, Write};

fn solution(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    let mut nums = buf.trim().split(' ').map(str::parse::<usize>).flatten();
    let (n, m) = (nums.next().unwrap(), nums.next().unwrap());
    buf.clear();
    input.read_line(&mut buf);
    let line = buf.trim().as_bytes();
    let mut l = 0;
    let mut r = 0;
    let mut maxl = 0;
    let mut an = 0;
    let mut bn = 0;
    while r < n {
        if line[r] == b'a' {
            an += 1;
        } else {
            bn += 1;
        }
        if an <= m || bn <= m {
            r += 1;
        } else {
            maxl = maxl.max(r - l);
            if line[l] == b'a' {
                l += 1;
                an -= 1;
            } else {
                l += 1;
                bn -= 1;
            }
            r += 1;
        }

    }
    maxl = maxl.max(r-l);
    writeln!(output, "{}", maxl);
}

fn main() {
    solution(std::io::stdin().lock(), std::io::stdout());
}
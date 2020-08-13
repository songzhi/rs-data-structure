use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::str::FromStr;

#[inline]
fn min_distance(x: isize, y: isize, n: isize) -> isize {
    (x - y).abs().min(n - (x - y).abs()) + 1
}

fn solve(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    let mut nums = buf.trim().split(' ').map(isize::from_str).flatten();
    let (n, m, c) = (
        nums.next().unwrap(),
        nums.next().unwrap(),
        nums.next().unwrap(),
    );
    let mut map: HashMap<isize, Vec<isize>> = (1..=c).map(|i| (i, vec![])).collect();
    for i in 0..n {
        buf.clear();
        input.read_line(&mut buf);
        let mut colors = buf.trim().split(' ').map(isize::from_str).flatten();
        colors.next().unwrap();
        for c in colors {
            map.get_mut(&c).unwrap().push(i);
        }
    }
    let mut count = 0;
    for (c, b) in map {
        if b.is_empty() {
            continue;
        }
        for i in 0..b.len() - 1 {
            if min_distance(b[i], b[i + 1], n) <= m {
                count += 1;
                break;
            }
        }
    }
    writeln!(output, "{}", count);
}

fn main() {
    solve(std::io::stdin().lock(), std::io::stdout());
}

#[test]
fn test() {
    let input1 = r#"100 10 50
50 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50
1 49
0
0
0
0
0
0
0
0
50 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50
1 48
0
0
0
0
1 42
0
0
1 28
50 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50
0
0
0
0
0
1 39
0
0
1 47
50 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50
1 45
0
0
1 34
0
0
0
0
0
50 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50
0
0
0
0
0
0
0
0
1 26
50 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50
0
0
0
0
0
0
0
0
0
50 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50
0
0
0
0
1 25
0
0
1 33
1 1
50 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50
0
0
0
0
0
0
0
0
0
50 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50
1 27
0
1 41
0
1 13
0
1 29
0
0
50 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50
0
0
0
0
0
2 5 11
0
1 50
0
"#;
    let output1 = r#"19
"#;
    crate::test_it(|x, y| solve(x, y), input1, output1)
}

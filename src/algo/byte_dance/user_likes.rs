use std::io::{BufRead, Write};
use std::str::FromStr;

fn solve(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    buf.clear();
    input.read_line(&mut buf);
    let mut likes = buf
        .trim()
        .split(' ')
        .map(isize::from_str)
        .flatten()
        .enumerate()
        .map(|(x, y)| (y, x as isize))
        .collect::<Vec<_>>();
    likes.sort();
    buf.clear();
    input.read_line(&mut buf);
    let q = buf.trim().parse::<usize>().unwrap();
    for _ in 0..q {
        buf.clear();
        input.read_line(&mut buf);
        let mut nums = buf.trim().split(' ').map(isize::from_str).flatten();
        let (l, r, k) = (
            nums.next().unwrap() - 1,
            nums.next().unwrap() - 1,
            nums.next().unwrap(),
        );
        if let Ok(index) = likes.binary_search_by_key(&k, |(x, _)| *x) {
            let mut lower_bound = index as isize;
            while lower_bound >= 0
                && likes[lower_bound as usize].0 == k
                && likes[lower_bound as usize].1 >= l
            {
                lower_bound -= 1;
            }
            let mut upper_bound = index;
            while upper_bound < likes.len()
                && likes[upper_bound].0 == k
                && likes[upper_bound].1 <= r
            {
                upper_bound += 1;
            }
            writeln!(output, "{}", upper_bound.saturating_sub((lower_bound + 1) as usize));
        } else {
            writeln!(output, "0");
        }
    }
}

fn main() {
    solve(std::io::stdin().lock(), std::io::stdout());
}

#[test]
fn test() {
    crate::test_it(
        |x, y| solve(x, y),
        r#"5
1 2 3 3 5
3
1 2 1
2 4 5
3 5 3"#,
        r#"1
0
2
"#,
    )
}

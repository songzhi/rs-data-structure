use std::io::{BufRead, Write};
use std::str::FromStr;

// 将字典序视作一个树，寻找m次则循环m次来找寻结果
// 如果在这个区间内则M在这个区间内查找，否则让梯度乘以10向上查找，直到找寻一个区间内，让i+1一个一个查找
// 第一步while循环是判断是否查到这个位置，第二次则是写出num在这个区间内有多少个数
// 本题不用构造一颗字典序树，却用到树的概念
// 以十个十个数为区间计算
fn solve(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    let mut nums = buf.trim_end().split(' ').map(usize::from_str).flatten();
    let (n, mut m) = (nums.next().unwrap(), nums.next().unwrap());
    let mut i = 1;
    m -= 1;
    while m != 0 {
        let mut start = i;
        let mut end = i + 1;
        let mut num = 0;
        while start <= n {
            num += end.min(n + 1) - start;
            start *= 10;
            end *= 10;
        }
        if num > m {
            i *= 10;
            m -= 1;
        } else {
            m -= num;
            i += 1;
        }
    }
    writeln!(output, "{}", i);
}

fn main() {
    solve(std::io::stdin().lock(), std::io::stdout())
}

#[test]
fn test() {
    crate::test_it(|x,y|solve(x,y), "11 4\n", "2\n");
}
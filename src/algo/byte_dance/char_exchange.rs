use std::io::{BufRead, Write};

fn max_chars(char: &[usize], m: usize) -> usize {
    let n = char.len();
    let mut dp = vec![vec![0; n]; n];
    // 计算当前字母，相邻两个位置移动到一起所需要的次数，存放到m*m的矩阵dp中,
    // dp[0][1]表示当前字母第一次出现移动到第二次出现需要的次数,
    // dp[0][1] = abs(v[1]-v[0])-1
    for i in 0..n - 1 {
        dp[i][i + 1] = char[i + 1] - char[i] - 1;
    }
    // 最快实现将相同字母排在一起的方法就是从两边往中间靠，dp[i][j]是当前状态，
    // 表示第i个字母到第j个字母移动的次数，那么当前状态只跟dp[i+1][j-1]有关，
    // 由于将i位置与j位置移动到一起需要(v[j]-v[i]-1)次，但是由于区间内已经有了移动好的(j-i-1)个字母，
    // 所以可以少移动这么多次，故需要减去这个数字,
    // 所以状态转移方程可以写作：dp[i][j] = dp[i+1][j-1]+(v[j]-v[i]-1)-(j-i-1)
    for j in 2..n {
        for i in 0..n - j {
            let row = i;
            let col = i + j;
            dp[row][col] =
                dp[row + 1][col - 1] + char[col] - char[row] - (col - row) as usize;
        }
    }
    let mut max = 0;
    // 在这个最小次数满足满足约束条件的前提下，筛选出最大的连续字母的个数
    for i in 0..n {
        for j in i..n {
            if dp[i][j] <= m {
                max = max.max(j - i + 1);
            }
        }
    }
    max
}

fn solve(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    let mut split = buf.trim().split(' ');
    let s = split.next().unwrap();
    let m: usize = split.next().unwrap().parse().unwrap();
    // 用来保存26个字母及其出现的位置
    let mut chars = vec![vec![]; 26];
    for (i, &c) in s.as_bytes().iter().enumerate() {
        chars[(c - b'a') as usize].push(i);
    }
    let mut max = 0;
    for char in chars {
        max = max.max(max_chars(char.as_slice(), m));
    }
    writeln!(output, "{}", max);
}

fn main() {
    solve(std::io::stdin().lock(), std::io::stdout());
}

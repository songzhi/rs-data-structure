use std::io::{BufRead, Write};
use std::str::FromStr;

fn solve(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    let n: usize = buf.trim_end().parse().unwrap();
    buf.clear();
    input.read_line(&mut buf);
    let mut nums: Vec<usize> = buf
        .trim_end()
        .split(' ')
        .map(usize::from_str)
        .flatten()
        .collect();
    let mut sum = vec![nums[0]];
    for i in 1..n {
        sum.push(sum[i - 1] + nums[i]);
    }
    let mut st = vec![0];
    let mut ans = 0;
    for i in 1..n {
        while let Some(top) = st.pop() {
            // 栈非空且栈顶元素大于当前元素，为了维持单调递增栈，弹出栈顶元素
            // 栈顶元素为区间最小值，出栈。左边界为栈顶或者空
            if nums[top] < nums[i] {
                st.push(top);
                break;
            }
            ans =
                ans.max((sum[i - 1] - st.last().copied().map(|c| sum[c]).unwrap_or(0)) * nums[top]);
        }
        st.push(i);
    }
    writeln!(output, "{}", ans);
}

fn main() {
    solve(std::io::stdin().lock(), std::io::stdout());
}

#[test]
fn test() {
    let input1 = "3\n6 2 1";
    let output1 = "36\n";
    crate::test_it(|x, y| solve(x, y), input1, output1);
}

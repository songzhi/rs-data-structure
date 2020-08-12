use std::io::{BufRead, Write};

// FIXME: can't be compiled
fn dp(costs: &[Vec<usize>], i: usize, mut nodes: impl Iterator<Item = usize> + Clone) -> usize {
    if nodes.clone().count() == 1 {
        let node = nodes.next().unwrap();
        return costs[i][node] + costs[node][0];
    }
    let nodes_copy = nodes.clone();
    nodes
        .enumerate()
        .map(|(i, node)| {
            costs[i][node] + dp(costs, node, nodes_copy.clone().filter(|n| n.ne(&node)))
        })
        .min()
        .unwrap_or(0)
}

fn solution(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf).ok();
    let n: usize = buf.trim().parse().unwrap();
    let mut costs: Vec<Vec<usize>> = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        input.read_line(&mut buf).ok();
        costs.push(
            buf.trim()
                .split(' ')
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
        );
    }
    writeln!(output, "{}", dp(&costs, 0, (1..n))).ok();
}

fn main() {
    solution(std::io::stdin().lock(), std::io::stdout());
}

#[test]
fn test() {
    crate::test_it(|x,y|solution(x,y), "","");
}

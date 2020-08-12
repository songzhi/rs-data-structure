use std::collections::VecDeque;
use std::io::{BufRead, Write};

fn main() {
    solve(std::io::stdin().lock(), std::io::stdout());
}

fn solve(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    let mut nums = buf.trim().split(' ').map(str::parse::<usize>).flatten();
    let (n, m) = (nums.next().unwrap(), nums.next().unwrap());
    let mut chars = vec![Vec::with_capacity(m); n];
    let mut start_x = 0;
    let mut start_y = 0;
    let mut box_x = 0;
    let mut box_y = 0;
    for (i, line) in chars.iter_mut().enumerate() {
        buf.clear();
        input.read_line(&mut buf);
        line.extend_from_slice(buf.trim().as_bytes());
        if let Some(y) = line.iter().position(|&c| c == b'S') {
            start_x = i as isize;
            start_y = y as isize;
        }
        if let Some(y) = line.iter().position(|&c| c == b'0') {
            box_x = i as isize;
            box_y = y as isize;
        }
    }
    writeln!(
        output,
        "{}",
        bfs_min_step(chars.as_slice(), start_x, start_y, box_x, box_y)
    );
}

#[derive(Copy, Clone)]
struct Node {
    x: isize,
    y: isize,
    bx: isize,
    by: isize,
    steps: isize,
}

impl Node {
    #[inline]
    fn new(x: isize, y: isize, bx: isize, by: isize) -> Self {
        Self {
            x,
            y,
            bx,
            by,
            steps: 0,
        }
    }
    #[inline]
    fn is_vertical(&self) -> bool {
        self.y == self.by
    }
    #[inline]
    fn is_horizontal(&self) -> bool {
        self.x == self.bx
    }
}

fn bfs_min_step(
    chars: &[Vec<u8>],
    start_x: isize,
    start_y: isize,
    box_x: isize,
    box_y: isize,
) -> isize {
    let start = Node::new(start_x, start_y, box_x, box_y);
    let n = chars.len();
    let m = chars[0].len();
    let mut is_visited = vec![vec![vec![vec![false; m]; n]; m]; n];
    let n = n as isize;
    let m = m as isize;
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(cur) = queue.pop_front() {
        let mut bx = cur.bx;
        let mut by = cur.by;
        for dir in dirs.iter() {
            if cur.is_vertical() {
                bx = if cur.x + dir.0 == cur.bx {
                    cur.bx + dir.0
                } else {
                    cur.bx
                };
            }
            if cur.is_horizontal() {
                by = if cur.y + dir.1 == cur.by {
                    cur.by + dir.1
                } else {
                    cur.by
                };
            }
            let mut next = Node::new(cur.x + dir.0, cur.y + dir.1, bx, by);
            if next.x < 0
                || next.x >= n
                || next.y < 0
                || next.y >= m
                || chars[next.x as usize][next.y as usize] == b'#'
                || next.bx < 0
                || next.bx >= n
                || next.by < 0
                || next.by >= m
                || chars[next.bx as usize][next.by as usize] == b'#'
            {
                continue;
            }
            if !is_visited[next.x as usize][next.y as usize][next.bx as usize][next.by as usize] {
                is_visited[next.x as usize][next.y as usize][next.bx as usize][next.by as usize] =
                    true;
                next.steps = cur.steps + 1;
                if chars[next.bx as usize][next.by as usize] == b'E' {
                    return next.steps;
                }
                queue.push_back(next);
            }
        }
    }
    -1
}

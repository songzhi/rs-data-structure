use std::cmp::{Ordering, Reverse};
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::io::{BufRead, Write};
use std::str::FromStr;

fn main() {
    solve(std::io::stdin().lock(), std::io::stdout());
}

fn solve(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf);
    let mut nums = buf.trim().split(' ').map(usize::from_str).flatten();
    let (n, m, p) = (
        nums.next().unwrap(),
        nums.next().unwrap(),
        nums.next().unwrap(),
    );
    let mut ready_ideas = BinaryHeap::with_capacity(p);
    let mut ideas = vec![];
    for i in 0..p {
        buf.clear();
        input.read_line(&mut buf);
        let mut idea = Idea::from_str(buf.as_str()).unwrap();
        idea.id = i;
        ideas.push(idea);
    }
    ideas.sort_by_key(|i| i.created_at);
    let mut ideas = ideas.into_iter().peekable();
    let mut ans = Vec::with_capacity(p);
    let mut programmers = BinaryHeap::from(vec![Reverse(1); m]);
    let mut tick = 1;
    while !(ready_ideas.is_empty() && ideas.peek().is_none()) {
        while let Some(idea) = ideas.peek() {
            if idea.created_at <= tick {
                ready_ideas.push(*idea);
                ideas.next();
            } else {
                break;
            }
        }
        while let Some(idea) = ready_ideas.pop() {
            let programmer = programmers.pop().unwrap().0;
            if programmer > tick {
                programmers.push(Reverse(programmer));
                ready_ideas.push(idea);
                break;
            }
            let complete_time = tick + idea.time_needed;
            ans.push((idea.id, complete_time));

            programmers.push(Reverse(complete_time));
        }
        tick += 1;
    }
    ans.sort_by_key(|x|x.0);
    for (_, t) in ans {
        writeln!(output, "{}", t);
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Idea {
    id: usize,
    pm_id: usize,
    created_at: usize,
    priority: usize,
    time_needed: usize,
}

impl FromStr for Idea {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.trim().split(' ').map(usize::from_str).flatten();
        Ok(Self {
            id: 0,
            pm_id: nums.next().unwrap(),
            created_at: nums.next().unwrap(),
            priority: nums.next().unwrap(),
            time_needed: nums.next().unwrap(),
        })
    }
}

impl PartialOrd for Idea {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.priority
                .cmp(&other.priority)
                .then(self.time_needed.cmp(&other.time_needed).reverse())
                .then(self.created_at.cmp(&other.created_at).reverse()),
        )
    }
}

impl Ord for Idea {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[test]
fn test() {
    crate::test_it(
        |x, y| solve(x, y),
        r#"2 2 5
1 1 1 2
1 2 1 1
1 3 2 2
2 1 1 2
2 3 5 5"#,
        r#"3
4
5
3
9
"#,
    )
}

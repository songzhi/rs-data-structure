use random::Source;
use std::fmt::{self, Display, Formatter};
use std::collections::HashSet;


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    East = 0,
    South,
    West,
    North,
}

impl Direction {
    fn next_direction(&self) -> Option<Self> {
        match self {
            Direction::East => Some(Direction::South),
            Direction::South => Some(Direction::West),
            Direction::West => Some(Direction::North),
            Direction::North => None
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Block {
    pos: (usize, usize),
    direction: Direction,
}

impl Block {
    fn new(pos: (usize, usize)) -> Self {
        Self {
            pos,
            direction: Direction::East,
        }
    }
    fn next_block(&self) -> Self {
        let (x, y) = self.pos;
        let next_pos = match self.direction {
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
            Direction::North => (x, y - 1)
        };
        Self::new(next_pos)
    }
}

pub struct Maze {
    layout: Vec<Vec<bool>>,
}


impl Maze {
    pub fn default() -> Self {
        Self::new(16)
    }

    pub fn new(len: usize) -> Self {
        let mut source = random::default().seed([42, 69]);
        let mut sequence = source.iter::<i8>();
        let mut layout = vec![vec![false; len]; len];
        for row in layout.iter_mut() {
            for elem in row.iter_mut() {
                *elem = sequence.next().unwrap() >= 0;
            }
        }
        Self {
            layout
        }
    }

    pub fn has_path(&self, start: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        self.has_path_with_visited(start, &mut visited)
    }

    fn has_path_with_visited(&self, start: (usize, usize), visited: &mut HashSet<(usize, usize)>)
                             -> Option<Vec<(usize, usize)>> {
        let mut stack = Vec::new();
        let mut current_block = Block::new(start);
        loop {
            if self.is_passable(current_block.pos) && visited.insert(current_block.pos) { // passable and not visited before
                stack.push(current_block);
                if self.is_in_edge(current_block.pos) {
                    break Some(stack.iter().map(|block| block.pos).collect());
                }
                current_block = current_block.next_block();
            } else {
                let mut iter = stack.into_iter();
                iter.by_ref().take_while(|block| block.direction == Direction::North).count();
                stack = iter.collect();
                if let Some(mut block) = stack.pop() {
                    block.direction = block.direction.next_direction().unwrap();
                    current_block = block.next_block();
                    stack.push(block);
                }
            }
            if stack.is_empty() {
                break None;
            }
        }
    }
    pub fn len(&self) -> usize {
        self.layout.len()
    }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
    pub fn is_in_edge(&self, (x, y): (usize, usize)) -> bool {
        x == 0 || x == self.len() || y == 0 || y == self.len()
    }
    pub fn is_passable(&self, (x, y): (usize, usize)) -> bool {
        self.layout[x][y]
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for row in self.layout.iter() {
            for &elem in row.iter() {
                if elem {
                    write!(f, ".")? // passable
                } else {
                    write!(f, "#")?
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}
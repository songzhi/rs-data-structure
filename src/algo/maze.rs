use random::Source;

pub struct Maze {
    layout: Vec<Vec<bool>>,
}


impl Maze {
    pub fn new() -> Self {
        Self::with_len(16)
    }
    pub fn with_len(len: usize) -> Self {
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
}
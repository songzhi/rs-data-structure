use random::Source;

pub struct Maze {
    layout: [[bool; 16]; 16],
}


impl Maze {
    pub fn new() -> Self {
        let mut source = random::default().seed([42, 69]);
        let mut sequence = source.iter::<i8>();
        let mut layout = [[false; 16]; 16];
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
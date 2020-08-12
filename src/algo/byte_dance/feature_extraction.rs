use std::collections::HashMap;
use std::io::{BufRead, Read, Write};
use std::ops::Neg;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Feature(usize, usize);

fn longest_feature_motion(frames: &[Vec<Feature>]) -> usize {
    let mut map: HashMap<Feature, isize> = HashMap::new();
    let mut max = 1;
    for frame in frames {
        for feature in frame {
            if let Some(count) = map.get_mut(feature) {
                *count = -(*count + 1);
            } else {
                map.insert(*feature, -1);
            }
        }
        map.retain(|_, v| v.is_negative());
        map.iter_mut().for_each(|(_, v)| {
            *v = v.neg();
            max = max.max(*v);
        });
    }
    max as usize
}

fn solution(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf).ok();
    let n: usize = buf.trim().parse().unwrap();
    let mut frames = vec![];
    for _ in 0..n {
        buf.clear();
        frames.clear();
        input.read_line(&mut buf).ok();
        let m: usize = buf.trim().parse().unwrap();
        for _ in 0..m {
            buf.clear();
            input.read_line(&mut buf).ok();
            let mut nums = buf.trim().split(' ').map(|s| s.parse::<usize>().unwrap());
            nums.next();
            let mut features = vec![];
            while let (Some(x), Some(y)) = (nums.next(), nums.next()) {
                features.push(Feature(x, y));
            }
            frames.push(features);
        }
        writeln!(output, "{}", longest_feature_motion(frames.as_slice()));
    }
}

fn main() {
    solution(std::io::stdin().lock(), std::io::stdout());
}

#[test]
fn test() {
    let input = r#"1
8
2 1 1 2 2
2 1 1 1 4
2 1 1 2 2
2 2 2 1 4
0
0
1 1 1
1 1 1"#;
    let correct_output = "3\n";
    let mut output = vec![];
    solution(input.as_bytes(), &mut output);
    assert_eq!(String::from_utf8(output).unwrap(), correct_output);
}

//! 假定每一个国家在地图上是一个连通域，并且有相邻边界 线的两个国家必须用不同的颜色，问是否只要四种颜色就可完成着色。
//! 现在给定一张地 图，要求对这张地图上的国家用不超过四种的颜色进行染色。
//! 要求建立地图的邻接矩阵存储结构，输入国家的个数和相邻情况，输出每个国家的颜色代码。
//!


use std::collections::HashMap;
use std::fmt::{Display, Formatter, self};

struct Graph {
    adj_matrix: Vec<Vec<bool>>
}

impl Graph {
    fn new(adj_matrix: Vec<Vec<bool>>) -> Self {
        Self {
            adj_matrix
        }
    }
    fn get_neighbors(&self, i: usize) -> Vec<usize> {
        self.adj_matrix[i].iter().enumerate()
            .filter(|&(_, is_adj)| *is_adj)
            .map(|(j, _)| j).collect()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Color {
    Red = 0,
    Blue,
    Green,
    White,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Color::Red => "red",
            Color::Blue => "blue",
            Color::Green => "green",
            Color::White => "white"
        })
    }
}

impl Color {
    fn from_usize(color: usize) -> Self {
        match color {
            0 => Color::Red,
            1 => Color::Blue,
            2 => Color::Green,
            3 => Color::White,
            _ => unreachable!()
        }
    }
}

pub fn color_it(map: Vec<Vec<bool>>) -> Vec<(usize, Color)> {
    let vertex_count = map.len();
    let graph = Graph::new(map);
    let mut colored = HashMap::<usize, usize>::new();
    let mut stack = Vec::<usize>::new();
    let mut i = 0;
    let mut neighbor_colored = [false; 4];
    while i < vertex_count {
        for j in graph.get_neighbors(i) {
            if let Some(color) = colored.get(&j) {
                neighbor_colored[*color] = true;
            }
        }
        if let Some((next_color, _)) = neighbor_colored.iter()
            .enumerate().find(|&(_, c)| !*c) {
            colored.insert(i, next_color);
            stack.push(i);
            i += 1;
        } else {
            i = stack.pop().expect("stack is empty");
            colored.remove(&i);
        }
        neighbor_colored.iter_mut().for_each(|c| *c = false);
    }
    let mut res: Vec<(usize, Color)> = colored.iter().map(|(i, color)| (*i, Color::from_usize(*color))).collect();
    res.sort_by_key(|&(i, _)| i);
    res
}

#[allow(dead_code)]
fn test() {
    let adj_matrix = vec![
        vec![false, true, false, false, false, false, true],
        vec![true, false, true, true, true, true, true],
        vec![false, true, false, true, false, false, false],
        vec![false, true, true, false, true, false, false],
        vec![false, true, false, true, false, true, true],
        vec![false, true, false, false, true, false, true],
        vec![true, true, false, false, true, true, false]
    ];

    let res = color_it(adj_matrix);
    for (i, color) in res {
        println!("{}: {}", i, color);
    }
}

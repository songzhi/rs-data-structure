//! 鞍点问题: 若矩阵 A 中的某一元素 A[i,j]是第 i 行中的最小值，而又是第 j 列中的最 大值，则称 A[i,j]是矩阵 A 中的一个鞍点。
//!

pub fn saddle_points<T: Ord>(matrix: &Vec<Vec<T>>) -> Vec<(usize, usize)> {
    let row_mins: Vec<(usize, usize)> = matrix.iter().enumerate().map(|(i, row)| {
        (i, row.iter().enumerate().min_by_key(|&(_, elem)| elem).unwrap().0)
    }).collect();
    let is_max_in_col = |&(x, y): &(usize, usize)| {
        for i in 0..matrix.len() {
            if matrix[i][y] > matrix[x][y] {
                return false;
            }
        }
        true
    };
    row_mins.into_iter().filter(is_max_in_col).collect()
}
pub fn create_board_checkered(width: usize, height: usize) -> Vec<Vec<bool>> {
    (0..height)
        .map(|j| (0..width).map(|i| (i + j) % 2 == 0).collect())
        .collect()
}

pub fn create_board_empty(width: usize, height: usize) -> Vec<Vec<bool>> {
    (0..height)
        .map(|j| (0..width).map(|i| false).collect())
        .collect()
}

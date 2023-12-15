pub fn create_board_empty(width: usize, height: usize) -> Vec<Vec<bool>> {
    (0..height)
        .map(|_| (0..width).map(|_| false).collect())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_board_empty() {
        let board = create_board_empty(2, 3);

        assert_eq!(board.len(), 3);
        assert_eq!(board[0].len(), 2);
        assert_eq!(board[2][1], false);
    }
}
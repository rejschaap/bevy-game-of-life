pub fn create_board_empty(width: usize, height: usize) -> Vec<Vec<bool>> {
    (0..height)
        .map(|_| (0..width).map(|_| false).collect())
        .collect()
}

pub fn update_board(board: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    board
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_board_empty() {
        let board = create_board_empty(1, 2);

        assert_eq!(board.len(), 2);
        assert_eq!(board[0].len(), 1);

        assert!(!board[0][0]);
        assert!(!board[1][0]);
    }

    #[test]
    fn test_rule_underpopulation_zero() {
        // Rule 1: Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        // Create a cell that is surrounded by dead cells
        let mut board = create_board_empty(3, 3);
        board[1][1] = true;

        // Update the board
        board = update_board(board);

        // Cell should have died
        assert!(!board[1][1])
    }

    #[test]
    fn test_rule_underpopulation_one() {
        // Rule 1: Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        // Create a cell with a single live neighbour
        let mut board = create_board_empty(3, 3);
        board[1][1] = true;
        board[1][2] = true;

        // Update the board
        board = update_board(board);

        // Cell should have died
        assert!(!board[1][1])
    }

    #[test]
    fn test_rule_live_two() {
        // Rule 2: Any live cell with two or three live neighbours lives on to the next generation.
        // Create a cell with two live neighbours
        let mut board = create_board_empty(3, 3);
        board[1][1] = true;
        board[1][2] = true;
        board[2][2] = true;

        // Update the board
        board = update_board(board);

        // Cell should stay alive
        assert!(board[1][1])
    }

    #[test]
    fn test_rule_live_three() {
        // Rule 2: Any live cell with two or three live neighbours lives on to the next generation.
        // Create a cell with three live neighbours
        let mut board = create_board_empty(3, 3);
        board[1][1] = true;

        board[1][2] = true;
        board[2][2] = true;
        board[0][0] = true;

        // Update the board
        board = update_board(board);

        // Cell should stay alive
        assert!(board[1][1])
    }

    #[test]
    fn test_rule_overpopulation_four() {
        // Rule 3: Any live cell with more than three live neighbours dies, as if by overpopulation.
        // Create a cell with four live neighbours
        let mut board = create_board_empty(3, 3);
        board[1][1] = true;

        board[1][2] = true;
        board[2][2] = true;
        board[0][0] = true;
        board[1][0] = true;

        // Update the board
        board = update_board(board);

        // Cell should be dead
        assert!(!board[1][1])
    }

    #[test]
    fn test_rule_reproduction() {
        // Rule 4: Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        // Create a cell with three live neighbours
        let mut board = create_board_empty(3, 3);
        board[1][1] = false;

        board[1][2] = true;
        board[2][2] = true;
        board[0][0] = true;

        // Update the board
        board = update_board(board);

        // Cell should be alive
        assert!(board[1][1])
    }
}

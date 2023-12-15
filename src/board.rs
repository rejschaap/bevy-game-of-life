pub fn create_board_empty(width: usize, height: usize) -> Vec<Vec<bool>> {
    (0..height)
        .map(|_| (0..width).map(|_| false).collect())
        .collect()
}

pub fn create_board_with_glider(width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut board = create_board_empty(width, height);

    board[1][0] = true;
    board[2][1] = true;
    board[0][2] = true;
    board[1][2] = true;
    board[2][2] = true;

    board
}

pub fn update_board(board: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    assert!(board.len() > 0);

    let height = board.len();
    let width = board[0].len();

    (0..height)
        .map(|j| (0..width).map(|i| next_cell_state(&board, i, j)).collect())
        .collect()
}

fn next_cell_state(board: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    let alive = board[y][x];
    let count = count_live_neighbours(board, x, y);

    if alive && (count == 2 || count == 3) {
        return true;
    }

    if !alive && count == 3 {
        return true;
    }

    return false;
}

fn count_live_neighbours(board: &Vec<Vec<bool>>, x: usize, y: usize) -> i32 {
    let mut count = 0;

    for j in -1..2 {
        for i in -1..2 {
            let index_y = (y as i32 + j).rem_euclid(board.len() as i32) as usize;
            let index_x = (x as i32 + i).rem_euclid(board[index_y].len() as i32) as usize;

            if index_x == x && index_y == y {
                // Don't count the cell itself
                continue;
            }

            let alive = board[index_y][index_x];

            if alive {
                count += 1;
            }
        }
    }

    return count;
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
    fn test_count_neighbours_three() {
        // Create a cell with three live neighbours
        let mut board = create_board_empty(3, 3);
        board[1][1] = true;

        board[1][2] = true;
        board[2][2] = true;
        board[0][0] = true;

        assert_eq!(count_live_neighbours(&board, 1, 1), 3);
        assert_eq!(count_live_neighbours(&board, 0, 0), 3);
    }

    #[test]
    fn test_rule_underpopulation_zero() {
        // Rule 1: Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        // Create a cell that is surrounded by dead cells
        let mut board = create_board_empty(3, 3);
        board[1][1] = true;

        // Update the board
        board = update_board(&board);

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
        board = update_board(&board);

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
        board = update_board(&board);

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
        board = update_board(&board);

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
        board = update_board(&board);

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
        board = update_board(&board);

        // Cell should be alive
        assert!(board[1][1])
    }
}

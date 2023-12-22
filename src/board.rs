use rand::Rng;

#[derive(Default)]
pub struct Board {
    cells: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn empty(width: usize, height: usize) -> Board {
        let cells = (0..height)
            .map(|_| (0..width).map(|_| false).collect())
            .collect();

        Board::new(cells, width, height)
    }

    pub fn new(cells: Vec<Vec<bool>>, width: usize, height: usize) -> Board {
        Board {
            cells,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn is_alive(&self, i: usize, j: usize) -> bool {
        self.cells[j][i]
    }

    pub fn rows(&self) -> impl Iterator<Item = &Vec<bool>> {
        self.cells.iter()
    }

    pub fn with_gliders(width: usize, height: usize) -> Board {
        let mut board = Board::empty(width, height);
        board.add_gliders(10);
        board
    }

    pub fn add_gliders(&mut self, count: i32) {
        let mut rng = rand::thread_rng();
        assert!(!self.cells.is_empty());

        for _ in 0..count {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);

            self.add_glider(x, y);
        }
    }

    pub fn add_glider(&mut self, x: usize, y: usize) {
        self.set_alive(x, 1 + y);
        self.set_alive(1 + x, 2 + y);
        self.set_alive(2 + x, y);
        self.set_alive(2 + x, 1 + y);
        self.set_alive(2 + x, 2 + y);
    }

    pub fn set_alive(&mut self, x: usize, y: usize) {
        let i = x % self.width;
        let j = y % self.height;

        self.cells[j][i] = true;
    }

    pub fn update(&self) -> Board {
        assert!(!self.is_empty());

        let cells = (0..self.height)
            .map(|j| {
                (0..self.width)
                    .map(|i| self.next_cell_state(i, j))
                    .collect()
            })
            .collect();

        Board::new(cells, self.width, self.height)
    }

    fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    fn next_cell_state(&self, x: usize, y: usize) -> bool {
        let alive = self.cells[y][x];
        let count = self.count_live_neighbours(x, y);

        if alive && (count == 2 || count == 3) {
            return true;
        }

        if !alive && count == 3 {
            return true;
        }

        false
    }

    fn count_live_neighbours(&self, x: usize, y: usize) -> i32 {
        let mut count = 0;

        for j in -1..2 {
            for i in -1..2 {
                let index_x = (x as i32 + i).rem_euclid(self.width as i32) as usize;
                let index_y = (y as i32 + j).rem_euclid(self.height as i32) as usize;

                if index_x == x && index_y == y {
                    // Don't count the cell itself
                    continue;
                }

                let alive = self.cells[index_y][index_x];

                if alive {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_board_empty() {
        let board = Board::empty(1, 2);

        assert_eq!(board.cells.len(), 2);
        assert_eq!(board.cells[0].len(), 1);

        assert!(!board.cells[0][0]);
        assert!(!board.cells[1][0]);
    }

    #[test]
    fn test_count_neighbours_three() {
        // Create a cell with three live neighbours
        let mut board = Board::empty(3, 3);
        board.cells[1][1] = true;

        board.cells[1][2] = true;
        board.cells[2][2] = true;
        board.cells[0][0] = true;

        assert_eq!(board.count_live_neighbours(1, 1), 3);
        assert_eq!(board.count_live_neighbours(0, 0), 3);
    }

    #[test]
    fn test_rule_underpopulation_zero() {
        // Rule 1: Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        // Create a cell that is surrounded by dead cells
        let mut board = Board::empty(3, 3);
        board.cells[1][1] = true;

        // Update the board
        board = board.update();

        // Cell should have died
        assert!(!board.cells[1][1])
    }

    #[test]
    fn test_rule_underpopulation_one() {
        // Rule 1: Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        // Create a cell with a single live neighbour
        let mut board = Board::empty(3, 3);
        board.cells[1][1] = true;
        board.cells[1][2] = true;

        // Update the board
        board = board.update();

        // Cell should have died
        assert!(!board.cells[1][1])
    }

    #[test]
    fn test_rule_live_two() {
        // Rule 2: Any live cell with two or three live neighbours lives on to the next generation.
        // Create a cell with two live neighbours
        let mut board = Board::empty(3, 3);
        board.cells[1][1] = true;
        board.cells[1][2] = true;
        board.cells[2][2] = true;

        // Update the board
        board = board.update();

        // Cell should stay alive
        assert!(board.cells[1][1])
    }

    #[test]
    fn test_rule_live_three() {
        // Rule 2: Any live cell with two or three live neighbours lives on to the next generation.
        // Create a cell with three live neighbours
        let mut board = Board::empty(3, 3);
        board.cells[1][1] = true;

        board.cells[1][2] = true;
        board.cells[2][2] = true;
        board.cells[0][0] = true;

        // Update the board
        board = board.update();

        // Cell should stay alive
        assert!(board.cells[1][1])
    }

    #[test]
    fn test_rule_overpopulation_four() {
        // Rule 3: Any live cell with more than three live neighbours dies, as if by overpopulation.
        // Create a cell with four live neighbours
        let mut board = Board::empty(3, 3);
        board.cells[1][1] = true;

        board.cells[1][2] = true;
        board.cells[2][2] = true;
        board.cells[0][0] = true;
        board.cells[1][0] = true;

        // Update the board
        board = board.update();

        // Cell should be dead
        assert!(!board.cells[1][1])
    }

    #[test]
    fn test_rule_reproduction() {
        // Rule 4: Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        // Create a cell with three live neighbours
        let mut board = Board::empty(3, 3);
        board.cells[1][1] = false;

        board.cells[1][2] = true;
        board.cells[2][2] = true;
        board.cells[0][0] = true;

        // Update the board
        board = board.update();

        // Cell should be alive
        assert!(board.cells[1][1])
    }
}

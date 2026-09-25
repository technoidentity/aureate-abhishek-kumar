use std::fmt;

pub const EMPTY_CELL: char = '-';

pub struct Board {
    pub size: usize,
    pub cell: Vec<Vec<char>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "The board size must be greater than zero");

        Self {
            size,
            cell: vec![vec![EMPTY_CELL; size]; size],
        }
    }

    pub fn insert_new_symbol(&mut self, row: usize, col: usize, symbol: char) -> bool {
        if row >= self.size || col >= self.size {
            println!("Row and column must be between 1 and {}", self.size);
            return false;
        }

        if self.cell[row][col] != EMPTY_CELL {
            println!("That cell is already occupied");
            return false;
        }

        self.cell[row][col] = symbol;
        true
    }

    pub fn has_winner(&self, symbol: char) -> bool {
        let row_win = self
            .cell
            .iter()
            .any(|row| row.iter().all(|&cell| cell == symbol));

        let column_win = (0..self.size)
            .any(|column| (0..self.size).all(|row| self.cell[row][column] == symbol));

        let main_diagonal_win = (0..self.size).all(|index| self.cell[index][index] == symbol);
        let opposite_diagonal_win = (0..self.size)
            .all(|index| self.cell[index][self.size - 1 - index] == symbol);

        row_win || column_win || main_diagonal_win || opposite_diagonal_win
    }
}

impl fmt::Display for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cell {
            for cell in row {
                write!(formatter, "{cell} ")?;
            }
            writeln!(formatter)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn detects_a_row_win() {
        let mut board = Board::new(3);
        for column in 0..3 {
            board.insert_new_symbol(1, column, 'X');
        }
        assert!(board.has_winner('X'));
    }

    #[test]
    fn detects_a_column_win() {
        let mut board = Board::new(3);
        for row in 0..3 {
            board.insert_new_symbol(row, 2, 'O');
        }
        assert!(board.has_winner('O'));
    }

    #[test]
    fn detects_both_diagonal_wins() {
        let mut main_diagonal = Board::new(3);
        let mut opposite_diagonal = Board::new(3);

        for index in 0..3 {
            main_diagonal.insert_new_symbol(index, index, 'X');
            opposite_diagonal.insert_new_symbol(index, 2 - index, 'O');
        }

        assert!(main_diagonal.has_winner('X'));
        assert!(opposite_diagonal.has_winner('O'));
    }
}

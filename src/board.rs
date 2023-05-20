use std::fs;

pub struct Board {
    board: Vec<Vec<u32>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: vec![vec![0; 9]; 9],
        }
    }

    pub fn read_from_file(&mut self, filename: &str) {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

        let mut row = 0;
        let mut col = 0;

        // iterate over each character in the file
        for c in contents.chars() {
            // if the character is a digit, add it to the board
            if c.is_digit(10) {
                self.board[row][col] = c.to_digit(10).unwrap();
                // move to the next column
                col += 1;
            }

            // if the character is a newline, move to the next row and reset the column
            if c == '\n' {
                row += 1;
                col = 0;
            }
        }
    }

    pub fn print_board(&mut self) {
        println!("\nUnsolved Board");
        for (row, line) in self.board.iter().enumerate() {
            // print a horizontal line every 3 rows
            if row % 3 == 0 && row != 0 {
                println!("------+-------+------");
            }

            for (col, num) in line.iter().enumerate() {
                // print a vertical line every 3 columns
                if col % 3 == 0 && col != 0 {
                    print!("| ");
                }

                print!("{} ", num);
            }

            // print a newline after each row
            println!();
        }

        if self.solver() {
            self.print_solved_board();
        } else {
            self.solver();
        }
    }

    pub fn print_solved_board(&self) {
        println!("\nSolved Board");
        for (row, line) in self.board.iter().enumerate() {
            if row % 3 == 0 && row != 0 {
                println!("------+-------+------");
            }

            for (col, num) in line.iter().enumerate() {
                if col % 3 == 0 && col != 0 {
                    print!("| ");
                }

                print!("{} ", num);
            }

            println!();
        }
    }

    pub fn is_valid(&mut self, number: u32, column: usize, row: usize) -> bool {
        // check if the same number is in the same column
        if self.board[column].contains(&number) {
            return false;
        }

        // check if the same number is in the same row
        for i in 0..9 {
            if self.board[i][row] == number {
                return false;
            }
        }

        // check if the same number is in the same box
        let box_x = column / 3;
        let box_y = row / 3;

        for i in 0..3 {
            for j in 0..3 {
                if self.board[box_x * 3 + i][box_y * 3 + j] == number {
                    return false;
                }
            }
        }

        true
    }

    pub fn solver(&mut self) -> bool {
        // backtracking algorithm

        // find the first empty cell
        // if there are no empty cells, return true
        // for each number from 1 to 9
        // if the number is valid in the cell
        // set the cell to the number

        for row in 0..9 {
            for column in 0..9 {
                if self.board[column][row] == 0 {
                    for number in 1..10 {
                        if self.is_valid(number, column, row) {
                            self.board[column][row] = number;

                            if self.solver() {
                                return true;
                            }

                            self.board[column][row] = 0;
                        }
                    }

                    return false;
                }
            }
        }

        true
    }
}

// test the current methods
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let board = Board::new();

        assert_eq!(board.board.len(), 9);
    }

    #[test]
    fn test_read_from_file() {
        let mut board = Board::new();
        board.read_from_file("board.txt");

        // test random coordinates
        assert_eq!(board.board[0][0], 5);
        assert_eq!(board.board[2][3], 0);
        assert_eq!(board.board[8][8], 9);
    }

    #[test]
    fn solver() {
        let mut board = Board::new();
        board.read_from_file("board.txt");

        assert_eq!(board.solver(), true);
    }
}

// A library for CLI minesweeper game

// M: mine but not found
// X: mine and found
// ?: not revealed
// certain number: number of mines around

use rand::Rng;

// a struct to store the board
pub struct Board {
    pub board: Vec<Vec<char>>,
    pub rows: usize,
    pub cols: usize,
    pub mines: usize,
}

// a function to create initialize every cell to be ?
pub fn init_board(rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut board = Vec::new();
    for _ in 0..rows {
        let mut row = Vec::new();
        for _ in 0..cols {
            row.push('?');
        }
        board.push(row);
    }
    board
}

// add given number of mines to the board
pub fn add_mines(board: &mut Board) {
    let mut rng = rand::thread_rng();
    for _ in 0..board.mines {
        let row = rng.gen_range(0..board.rows);
        let col = rng.gen_range(0..board.cols);
        if board.board[row][col] == 'M' {
            continue;
        }
        board.board[row][col] = 'M';
    }
}

// print the board unless the cell is M, print ?
pub fn print_board(board: &Board) {
    // print the column numbers
    print!("  ");
    for i in 0..board.cols {
        print!("{} ", i);
    }
    println!();
    let mut row_num = 0;
    for row in &board.board {
        // print the row numbers
        print!("{} ", row_num);
        for cell in row {
            if *cell == 'M' {
                print!("? ");
            } else {
                print!("{} ", cell);
            }
        }
        println!();
        row_num += 1;
    }
}

// calculate the number of mines (M or X) around the given cell
pub fn count_mines(board: &Board, row: usize, col: usize) -> usize {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let r = row as i32 + i;
            let c = col as i32 + j;
            if r < 0 || r >= board.rows as i32 || c < 0 || c >= board.cols as i32 {
                continue;
            }
            if board.board[r as usize][c as usize] == 'M'
                || board.board[r as usize][c as usize] == 'X'
            {
                count += 1;
            }
        }
    }
    count
}

// a function to reveal the cell
pub fn reveal(board: &mut Board, row: usize, col: usize) {
    // calculate how many M or X around the cell
    let count = count_mines(board, row, col);
    // mark the cell with the number of mines around
    board.board[row][col] = (count + '0' as usize) as u8 as char;
}

// a function to check if the game is over
pub fn is_over(board: &Board) -> bool {
    for row in &board.board {
        for cell in row {
            if *cell == '?' {
                return false;
            }
        }
    }
    true
}

// a function to reveal the cell that user clicked and check if the user hit a mine
pub fn click(board: &mut Board, row: usize, col: usize) -> bool {
    if board.board[row][col] == 'M' || board.board[row][col] == 'X' {
        return true;
    }
    reveal(board, row, col);
    false
}

// a function for user to initialize the board
pub fn user_init(rows: usize, cols: usize, mines: usize) -> Board {
    let mut board = Board {
        board: init_board(rows, cols),
        rows,
        cols,
        mines,
    };
    add_mines(&mut board);
    print_board(&board);
    board
}

// a function for user to click a cell
pub fn user_click(board: &mut Board, row: usize, col: usize) -> bool {
    if click(board, row, col) {
        println!("You hit a mine! You lose!");
        // print the board with all mines revealed
        for row in &mut board.board {
            for cell in row {
                if *cell == 'M' || *cell == 'X' {
                    print!("X ");
                }
                else {
                    print!("{} ", cell);
                }
            }
            println!();
        }
        return true;
    }
    print_board(board);
    if is_over(board) {
        println!("You win!");
        // print the board with all mines revealed
        for row in &mut board.board {
            for cell in row {
                if *cell == 'M' || *cell == 'X' || *cell == '?'{
                    print!("X ");
                }
                else {
                    print!("{} ", cell);
                }
            }
            println!();
        }
        return true;
    } else {
        println!("Continue!");
        return false;
    }
}

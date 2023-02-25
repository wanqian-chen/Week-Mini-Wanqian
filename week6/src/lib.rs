// A library for CLI minesweeper game

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
        if board.board[row][col] == 'X' {
            continue;
        }
        board.board[row][col] = 'X';
    }
}

// print the board
pub fn print_board(board: &Board) {
    for row in &board.board {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

// a function to reveal the cell which cannot be mined based on the given cell
pub fn reveal(board: &mut Board, rows: usize, cols: usize, row: usize, col: usize) {
    if row < 0 || row >= rows || col < 0 || col >= cols {
        return;
    }
    if board.board[row][col] != '?' {
        return;
    }
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            if row as i32 + i < 0 || row as i32 + i >= rows as i32 || col as i32 + j < 0
                || col as i32 + j >= cols as i32
            {
                continue;
            }
            if board.board[(row as i32 + i) as usize][(col as i32 + j) as usize] == 'X' {
                count += 1;
            }
        }
    }
    if count == 0 {
        board.board[row][col] = ' ';
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                reveal(board, rows, cols, (row as i32 + i) as usize, (col as i32 + j) as usize);
            }
        }
    } else {
        board.board[row][col] = (count + '0' as usize) as u8 as char;
    }
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
    if board.board[row][col] == 'X' {
        return true;
    }
    reveal(board, board.rows, board.cols, row, col);
    false
}

// a function for user to initialize the board
pub fn user_init(rows: usize, cols: usize, mines: usize) {
    let mut board = Board {
        board: init_board(rows, cols),
        rows,
        cols,
        mines,
    };
    add_mines(&mut board);
    print_board(&board);
}

// a function for user to click a cell
pub fn user_click(board: &mut Board, row: usize, col: usize) {
    if click(board, row, col) {
        println!("You hit a mine!");
        return;
    }
    print_board(board);
    if is_over(board) {
        println!("You win!");
    }
}
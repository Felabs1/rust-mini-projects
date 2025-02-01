#[derive(Clone, Copy, PartialEq, Debug)]
enum Player {
    X,
    O
}


// game board
struct TicTacToe {
    board: [[Option<Player>; 3]; 3], // 3x3 grid
    current_player: Player, // track whose turn it is
}


impl TicTacToe {
    fn new() -> Self {
        TicTacToe {
            board: [[None; 3]; 3], // initialize an empty board
            current_player: Player::X, // X always start
        }
    }

    // print the board to show the game state

    fn print_board(&self) {
        for row in &self.board {
            for &cell in row {
                match cell {
                    Some(Player::X) => print!(" X "),
                    Some(Player::O) => print!(" O "),
                    None => print!(" - "), // empty space
                }
            }
            println!()
        }
        println!()
    }

    // function to make a move on the board
    fn make_move(&mut self, row: usize, col: usize) -> bool {
        if row < 3 && col < 3 && self.board[row][col].is_none() {
            self.board[row][col] = Some(self.current_player);
            return true;
        }
        false // invalid move
    }

    // checking for a winner
    // player win if they complete a row, a column or a diagonal
    fn check_winner(&self) -> Option<Player> {
        let lines = [
            // Rows
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],

            // columns
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],

            // diagnals
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)]
        ];

        for &line in &lines {
            if let Some(player) = self.board[line[0].0][line[0].1] {
                if self.board[line[1].0][line[1].1] == Some(player) && self.board[line[2].0][line[2].1] == Some(player) {
                    return Some(player);
                }
            }
        }

        None
    }

    fn is_draw(&self) -> bool {
        self.board.iter().flatten().all(|&cell| cell.is_some())
    }

    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }
}

use std::io;

fn main() {
    
   let mut game = TicTacToe::new();

   loop {
    game.print_board();
    println!("player {:?}, enter row and column (0-2): ", game.current_player);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to load input");
    let nums: Vec<usize> = input.trim().split_whitespace().filter_map(|n| n.parse().ok()).collect();

    if nums.len() != 2 || !game.make_move(nums[0], nums[1]) {
        println!("invalid move try again");
        continue;
    }

    if let Some(winner) = game.check_winner() {
        game.print_board();
        println!("player {:?} wins!", winner);
        break;
    }

    if game.is_draw() {
        game.print_board();
        println!("it's a draw!");
        break;
    }

    game.switch_player();
   }
}

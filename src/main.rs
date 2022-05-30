use console::{Key, Term};

/// The board of a tic tac toe game.
/// <br><br>
/// Below is a visual of the cell indices (formatted board\[row]\[column]):
/// ```
/// [1] [2] [3] < -- 0
/// [4] [5] [6] < -- 1
/// [7] [8] [9] < -- 2
///  ^   ^   ^
///  |   |   |
///  0   1   2
/// ```
type Board = [[CellState; 3]; 3];

enum Player {
    Noughts,
    Crosses,
}

impl Player {
    fn to_letter(&self) -> char {
        match self {
            Player::Noughts => 'O',
            Player::Crosses => 'X'
        }
    }

    fn to_number(&self) -> i32 {
        match self {
            Player::Noughts => 2,
            Player::Crosses => 1
        }
    }
}

#[derive(PartialEq)]
enum CellState {
    Empty,
    Nought,
    Cross,
}

impl CellState {
    fn to_player(&self) -> Result<Player, &'static str> {
        match self {
            CellState::Empty => Err("Cell has not been played!"),
            CellState::Nought => Ok(Player::Noughts),
            CellState::Cross => Ok(Player::Crosses)
        }
    }
}

struct Game {
    board: Board,
    player: Player,
}

impl Game {
    fn switch(&mut self) {
        match self.player {
            Player::Noughts => self.player = Player::Crosses,
            Player::Crosses => self.player = Player::Noughts
        }
    }
}

fn get_cell(state: &CellState, cell: &mut usize) -> String {
    *cell += 1;

    return match state {
        CellState::Empty => { format!("[{}]", cell) }
        CellState::Nought => { String::from("[O]") }
        CellState::Cross => { String::from("[X]") }
    };
}

fn draw_board(board: &Board) {
    let mut i = 0;

    for row in board {
        println!(
            "{} {} {}",
            get_cell(&row[0], &mut i),
            get_cell(&row[1], &mut i),
            get_cell(&row[2], &mut i)
        );
    }
}

fn get_input(game: &mut Game, draw: bool) -> bool {
    let term = Term::stdout();

    if draw {
        println!("Please type a number to place an {letter}\nPlayer {number} ({letter}): ", letter = game.player.to_letter(), number = game.player.to_number());
    }

    match term.read_key() {
        Ok(result) => {
            match result {
                Key::Char(char) if char.is_digit(10) => {
                    match char.to_string().parse::<f32>() {
                        Ok(digit) => {
                            let mut index: Option<(usize, usize)> = None;

                            match digit.trunc() as i32 {
                                1 => index = Some((0, 0)),
                                2 => index = Some((0, 1)),
                                3 => index = Some((0, 2)),
                                4 => index = Some((1, 0)),
                                5 => index = Some((1, 1)),
                                6 => index = Some((1, 2)),
                                7 => index = Some((2, 0)),
                                8 => index = Some((2, 1)),
                                9 => index = Some((2, 2)),
                                _ => {}
                            }

                            match index {
                                Some(i) => {
                                    let row = i.0;
                                    let cell = i.1;

                                    match game.board[row][cell] {
                                        CellState::Empty => {
                                            match game.player {
                                                Player::Noughts => game.board[row][cell] = CellState::Nought,
                                                Player::Crosses => game.board[row][cell] = CellState::Cross
                                            }
                                            return true;
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }

    false
}

fn check_win(board: &Board) -> Option<Player> {
    // Vertical
    {
        let mut i_col = 0;

        for col in &board[0] {
            if col == &board[1][i_col] && col == &board[2][i_col] {
                match col.to_player() {
                    Ok(player) => return Some(player),
                    _ => {}
                }
            }

            i_col += 1;
        }
    }

    // Horizontal
    {
        for row in board {
            let mut all_equal = true;

            let mut prev = &row[0];

            for col in row {
                if col != prev {
                    all_equal = false;
                    break;
                } else {
                    prev = &col;
                };
            }

            if all_equal {
                match row[0].to_player() {
                    Ok(player) => return Some(player),
                    _ => {}
                }
            }
        }
    }

    // Diagonal
    {
        for row in [
            // 0usize to make sure that Rust knows all of these are usize
            [ [0,0], [1,1], [2,2] ],
            [ [0,2], [1,1], [2,0] ]
        ] {
            let mut all_equal = true;

            let mut prev = row[0].clone();

            for col in row {
                if board[col[0]][col[1]] != board[prev[0]][prev[1]] {
                    all_equal = false;
                    break;
                } else {
                    prev = col.clone();
                };
            }

            if all_equal {
                match board[row[0][0]][row[0][1]].to_player() {
                    Ok(player) => return Some(player),
                    _ => {}
                }
            }
        }
    }

    None
}

fn main() {
    let mut game = Game {
        board: [
            [
                CellState::Empty,
                CellState::Empty,
                CellState::Empty
            ],
            [
                CellState::Empty,
                CellState::Empty,
                CellState::Empty
            ],
            [
                CellState::Empty,
                CellState::Empty,
                CellState::Empty
            ]
        ],
        player: Player::Crosses,
    };

    let term = Term::stdout();

    loop {
        match term.clear_screen() {
            Err(_) => println!("\n==============================\n"),
            _ => {}
        }

        draw_board(&game.board);

        let mut draw = true;

        loop {

            if get_input(&mut game, draw) {
                break;
            } else {
                draw = false;
            }
        }

        match check_win(&game.board) {
            Some(player) => {
                match player {
                    Player::Noughts => {
                        println!("Noughts wins!");
                        break;
                    },
                    Player::Crosses => {
                        println!("Crosses wins!");
                        break;
                    }
                }
            }
            _ => {game.switch()}
        }
    }
}

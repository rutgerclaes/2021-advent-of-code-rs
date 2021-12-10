use ansi_term::Color::Blue;
use ansi_term::Color::Red;
use ansi_term::Style;
use either::{Either, Left, Right};
use im_rc::HashMap;
use im_rc::HashSet;
use itertools::FoldWhile;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::fmt::Display;
use utils::input::*;
use utils::output::*;
use utils::results::*;

#[macro_use]
extern crate log;

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let day = parse_day(file!());
    let file = file_name_from_args();
    let input = read_lines_from_file(&path_for_day(day, &file).unwrap()).unwrap();
    let (drawn_numbers, boards) = parse_input(input);

    info!(
        "Solution to part one: {}",
        display_result(part_one(&drawn_numbers, &boards))
    );
    info!(
        "Solution to part two: {}",
        display_result(part_two(&drawn_numbers, &boards))
    );
}

fn part_one(numbers: &[u8], boards: &[Board]) -> Result<u64> {
    let result = numbers
        .iter()
        .fold_while(Left(Vec::from(boards)), |boards, number| {
            let updated_boars: Vec<Board> = boards
                .unwrap_left()
                .iter()
                .map(|b| b.select(number))
                .collect();
            updated_boars.iter().find(|b| b.is_winner()).map_or_else(
                || Continue(Left(updated_boars.clone())),
                |winner| Done(Right(winner.unmarked_sum() * *number as u64)),
            )
        });

    match result {
        Done(Right(score)) => Ok(score),
        Continue(_) => Err(AOCError::new_from_ref("No winner found")),
        Done(_) => panic!("Unreachable state"),
    }
}

fn part_two(numbers: &[u8], boards: &[Board]) -> Result<u64> {
    let result: FoldWhile<Either<Vec<Board>, (Board, u8)>> =
        numbers
            .iter()
            .fold_while(Left(Vec::from(boards)), |boards, number| match boards {
                Left(multiple_boards) => {
                    let updated_boards: Vec<Board> = multiple_boards
                        .iter()
                        .filter_map(|board| Some(board.select(number)).filter(|b| !b.is_winner()))
                        .collect();

                    if updated_boards.len() == 1 {
                        debug!(
                            "Only one board left after applying {}:\n{}",
                            number, updated_boards[0]
                        );
                        Continue(Right((updated_boards[0].clone(), *number)))
                    } else {
                        debug!("Still multiple boards left after applying {}", number);
                        for board in &updated_boards {
                            debug!("\n{}", board);
                        }
                        Continue(Left(updated_boards))
                    }
                }
                Right((board, _)) => {
                    let updated_board = board.select(number);
                    if updated_board.is_winner() {
                        debug!("Single board has won after {}:\n{}", number, board);
                        Done(Right((updated_board, *number)))
                    } else {
                        debug!("Single board has not won yet after {}:\n{}", number, board);
                        Continue(Right((updated_board, *number)))
                    }
                }
            });

    match result {
        Done(result) => result
            .map_right(|(board, number)| Ok(board.unmarked_sum() * number as u64))
            .right_or(Err(AOCError::new_from_ref(
                "Last board didn't win in the end",
            ))),
        Continue(_) => Err(AOCError::new_from_ref(
            "No single board left at end of sequence",
        )),
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Position {
    row: u8,
    column: u8,
}

impl Position {
    fn new(row: u8, column: u8) -> Position {
        Position { row, column }
    }
}

impl From<&(u8, u8)> for Position {
    fn from(t: &(u8, u8)) -> Self {
        Position::new(t.0, t.1)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "row: {}, col: {}", self.row, self.column)
    }
}

#[derive(Debug, Clone)]
struct Board {
    hits: HashSet<Position>,
    width: u8,
    height: u8,
    positions: HashMap<u8, Position>,
    numbers: HashMap<Position, u8>,
}

impl Board {
    fn new(numbers: &[Vec<u8>]) -> Board {
        let numbers_map: HashMap<Position, u8> = numbers
            .iter()
            .enumerate()
            .flat_map(move |(row_nb, row)| {
                row.iter().enumerate().map(move |(col_nb, &number)| {
                    (Position::new(row_nb as u8, col_nb as u8), number)
                })
            })
            .collect();
        let positions = numbers_map.iter().map(|(k, v)| (*v, k.clone())).collect();
        let hits: HashSet<Position> = HashSet::new();
        Board {
            hits,
            width: 5,
            height: 5,
            positions,
            numbers: numbers_map,
        }
    }

    fn get_position(&self, position: &Position) -> &u8 {
        self.numbers.get(position).unwrap()
    }

    fn select(&self, number: &u8) -> Board {
        match self.positions.get(number) {
            None => self.clone(),
            Some(position) => Board {
                hits: self.hits.update(position.clone()),
                width: self.width,
                height: self.height,
                positions: self.positions.clone(),
                numbers: self.numbers.clone(),
            },
        }
    }

    fn is_winner(&self) -> bool {
        (0..self.width)
            .any(|col| (0..self.height).all(|row| self.hits.contains(&Position::new(row, col))))
            || (0..self.height)
                .any(|row| (0..self.width).all(|col| self.hits.contains(&Position::new(row, col))))
    }

    fn unmarked_sum(&self) -> u64 {
        (0..self.width)
            .flat_map(|col| (0..self.height).map(move |row| Position::new(row, col)))
            .filter(|pos| !self.hits.contains(pos))
            .fold(0, |sum, pos| sum + *self.numbers.get(&pos).unwrap() as u64)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut result = std::result::Result::Ok(());
        for row in 0..self.height {
            for col in 0..self.width {
                result = result.and_then(|_| {
                    let position = Position::new(row, col);
                    let style = if self.hits.contains(&position) {
                        Style::new().bold().fg(Blue)
                    } else {
                        Style::new().fg(Red)
                    };
                    write!(
                        f,
                        "{}{}",
                        style.paint(format!("{:2}", self.get_position(&position))),
                        if col < self.width - 1 { " " } else { "\n" }
                    )
                });
            }
        }
        result
    }
}

fn parse_input(input: Vec<String>) -> (Vec<u8>, Vec<Board>) {
    let mut iterator = input.iter();
    let numbers = iterator
        .next()
        .unwrap()
        .split(',')
        .map(|string| string.parse::<u8>().unwrap())
        .collect();

    let boards = iterator
        .skip(1)
        .batching(|it| {
            let board: Vec<Vec<u8>> = it
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    line.split_whitespace()
                        .map(|nb| nb.parse::<u8>().unwrap())
                        .collect::<Vec<u8>>()
                })
                .collect();

            if board.is_empty() {
                None
            } else {
                Some(Board::new(&board))
            }
        })
        .collect();

    (numbers, boards)
}

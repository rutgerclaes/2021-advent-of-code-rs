#![feature(option_result_contains)]

use im_rc::Vector;
use itertools::Itertools;
use tailcall::tailcall;
use utils::input::*;
use utils::output::*;
use utils::results::*;

#[macro_use]
extern crate log;

fn main() {
    init_env_log();

    let day = parse_day(file!());
    let file = file_name_from_args();
    let lines: Vec<String> = read_lines_from_file(&path_for_day(day, &file).unwrap()).unwrap();

    info!("Solution to part one: {}", display_result(part_one(&lines)));
    info!("Solution to part two: {}", display_result(part_two(&lines)));
}

fn part_one(lines: &[String]) -> Result<usize> {
    Ok(lines
        .iter()
        .map(|line| match validate(line) {
            LineResult::Corrupt(chars) => chars.iter().map(illegal_char_points).sum(),
            _ => 0,
        })
        .sum())
}

fn part_two(lines: &[String]) -> Result<usize> {
    let scores: Vec<usize> = lines
        .iter()
        .filter_map(|line| match validate(line) {
            LineResult::Incomplete(chars) => {
                let line_score = chars
                    .iter()
                    .fold(0, |score, ch| score * 5 + required_char_points(ch));
                debug!(
                    "Found incomplete line: {}.  Requires {:?} added for a score of {}",
                    line, chars, line_score
                );
                Some(line_score)
            }
            _ => None,
        })
        .sorted()
        .collect();

    debug!("Scores: {:?}", scores);
    scores
        .get(scores.len() / 2)
        .copied()
        .ok_or_else(|| AOCError::new_from_ref("Error fetching middle result"))
}

fn validate(string: &str) -> LineResult {
    #[tailcall]
    fn validate(
        mut input: Vector<char>,
        mut stack: Vector<char>,
        mut illegal_chars: Vector<char>,
    ) -> LineResult {
        match input.pop_front() {
            Some(found_open) if is_open_char(found_open) => {
                stack.push_front(found_open);
                trace!(
                    "New opening char {}, adding to stack {:?}",
                    found_open,
                    stack
                );
                validate(input, stack, illegal_chars)
            }
            Some(found_closed) => match stack.pop_front() {
                None => panic!("Found a string where there are surplus close characters"),
                Some(last_open) if close_char_for(&last_open) == found_closed => {
                    trace!(
                        "Found {} and that corresponds to close for {}",
                        found_closed,
                        last_open
                    );
                    validate(input, stack, illegal_chars)
                }
                Some(last_open) => {
                    trace!(
                        "Found '{}' and expected '{}' (based on {})",
                        found_closed,
                        close_char_for(&last_open),
                        last_open
                    );
                    illegal_chars.push_back(found_closed);
                    validate(input, stack, illegal_chars)
                }
            },
            None => {
                if stack.is_empty() && illegal_chars.is_empty() {
                    LineResult::Valid
                } else if illegal_chars.is_empty() {
                    let required_chars = stack.iter().map(close_char_for).collect();
                    LineResult::Incomplete(required_chars)
                } else {
                    LineResult::Corrupt(illegal_chars)
                }
            }
        }
    }

    validate(string.chars().collect(), Vector::new(), Vector::new())
}

fn close_char_for(open_char: &char) -> char {
    match open_char {
        '{' => '}',
        '(' => ')',
        '[' => ']',
        '<' => '>',
        c => panic!("Unsupported open char '{}'", c),
    }
}

fn illegal_char_points(c: &char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        c => panic!("Unsupported illegal char '{}'", c),
    }
}

fn required_char_points(c: &char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        c => panic!("Unsupported illegal char '{}'", c),
    }
}

fn is_open_char(open_char: char) -> bool {
    matches![open_char, '{' | '<' | '(' | '[']
}

#[derive(Debug, PartialEq)]
enum LineResult {
    Valid,
    Incomplete(Vector<char>),
    Corrupt(Vector<char>),
}

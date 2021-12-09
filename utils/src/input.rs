use crate::results::AOCError;
use crate::results::Result;
use itertools::Itertools;
use std::env;
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub fn parse_day(main_file: &str) -> &str {
    main_file.trim_end_matches("/src/main.rs")
}

pub fn file_name_from_args() -> String {
    env::args().nth(1).unwrap_or_else(|| "puzzle".to_owned())
}

pub fn path_for_day(day: &str, file_name: &str) -> Result<Box<Path>> {
    let path = fs::canonicalize(Path::new(&format!("{}/input/{}.input", day, file_name)))?;
    Ok(path.into_boxed_path())
}

pub fn read_string_from_file(path: &Path) -> Result<String> {
    fs::read_to_string(path).map_err(AOCError::from)
}

pub fn read_lines_from_file<C: FromIterator<String>>(path: &Path) -> Result<C> {
    read_string_from_file(path)
        .map(|contents| contents.lines().map(|line| line.to_string()).collect())
}

pub fn parse_lines_from_file<C, I>(path: &Path) -> Result<C>
where
    I: FromStr + Debug,
    C: FromIterator<I>,
    I::Err: Error,
{
    let contents = fs::read_to_string(path)?;
    contents
        .lines()
        .map(|line| line.parse::<I>())
        .try_collect()
        .map_err(|err| AOCError::new(err.to_string()))
}

pub fn parse_string_from_file<I>(path: &Path) -> Result<I>
where
    I: FromStr + Debug,
    I::Err: Error,
{
    read_string_from_file(path)?
        .parse::<I>()
        .map_err(|err| AOCError::new(err.to_string()))
}

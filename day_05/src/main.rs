use im_rc::HashMap;
use itertools::Itertools;
use std::fmt::Display;
use std::str::FromStr;
use utils::input::*;
use utils::output::*;
use utils::results::*;

#[macro_use]
extern crate log;

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let day = parse_day(file!());
    let file = file_name_from_args();
    let lines: Vec<Line> = parse_lines_from_file(&path_for_day(day, &file).unwrap()).unwrap();

    info!("Solution to part one: {}", display_result(part_one(&lines)));
    info!("Solution to part two: {}", display_result(part_two(&lines)));
}

fn part_one(lines: &[Line]) -> Result<usize> {
    let map = lines
        .iter()
        .filter(|l| l.is_horizontal() || l.is_vertical())
        .fold(Map::empty(), |map, line| map.update(line));
    Ok(map.get_overlap_count())
}

fn part_two(lines: &[Line]) -> Result<usize> {
    let map = lines
        .iter()
        .fold(Map::empty(), |map, line| map.update(line));
    Ok(map.get_overlap_count())
}

#[derive(Debug)]
struct Map {
    point_count: HashMap<Point, usize>,
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}

impl Map {
    fn empty() -> Map {
        Map {
            point_count: HashMap::new(),
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.point_count.is_empty()
    }

    fn get_count(&self, point: &Point) -> usize {
        self.point_count.get(point).copied().unwrap_or(0)
    }

    fn get_overlap_count(&self) -> usize {
        self.point_count.values().filter(|&&c| c > 1).count() as usize
    }

    fn update(&self, line: &Line) -> Map {
        let (new_min_x, new_min_y, new_max_x, new_max_y) = if !self.is_empty() {
            (
                std::cmp::min(std::cmp::min(line.start.x, line.end.x), self.min_x),
                std::cmp::min(std::cmp::min(line.start.y, line.end.y), self.min_y),
                std::cmp::max(std::cmp::max(line.start.x, line.end.x), self.max_x),
                std::cmp::max(std::cmp::max(line.start.y, line.end.y), self.max_y),
            )
        } else {
            (
                std::cmp::min(line.start.x, line.end.x),
                std::cmp::min(line.start.y, line.end.y),
                std::cmp::max(line.start.x, line.end.x),
                std::cmp::max(line.start.y, line.end.y),
            )
        };

        let new_counts = line
            .points()
            .fold(self.point_count.clone(), |counts, point| {
                counts.update_with(point, 1, |a, b| a + b)
            });

        Map {
            point_count: new_counts,
            min_x: new_min_x,
            min_y: new_min_y,
            max_x: new_max_x,
            max_y: new_max_y,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for y in self.min_y..self.max_y + 1 {
            for x in self.min_x..self.max_x + 1 {
                let count = self.get_count(&Point::new(x, y));
                write!(
                    f,
                    "{}",
                    if count > 0 {
                        format!("{}", count)
                    } else {
                        ".".to_string()
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok( () )
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl FromStr for Point {
    type Err = AOCError;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let (x, y) = input
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .ok_or_else(|| AOCError::new(format!("Error parsing '{}'", input)))?;
        Ok(Point::new(x, y))
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    fn points(&self) -> PointIterator {
        PointIterator {
            start: self.start,
            end: self.end,
            index: 0,
        }
    }
}

impl IntoIterator for Line {
    type Item = Point;

    type IntoIter = PointIterator;

    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        self.points()
    }
}

struct PointIterator {
    start: Point,
    end: Point,
    index: usize,
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let dx: i16 = self.end.x as i16 - self.start.x as i16;
        let dy: i16 = self.end.y as i16 - self.start.y as i16;

        if (dx.abs() as usize) >= self.index || (dy.abs() as usize) >= self.index {
            let result = Point::new(
                (self.start.x as i16 + (dx.signum() * self.index as i16)) as usize,
                (self.start.y as i16 + (dy.signum() * self.index as i16)) as usize,
            );
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl FromStr for Line {
    type Err = AOCError;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let points: Vec<Point> = input
            .split(" -> ")
            .map(Point::from_str)
            .try_collect()?;
        let (start, end) = points
            .into_iter()
            .collect_tuple()
            .ok_or_else(|| AOCError::new(format!("Error parsing '{}'", input)))?;
        Ok(Line::new(start, end))
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

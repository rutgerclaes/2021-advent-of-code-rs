use im_rc::Vector;
use itertools::Itertools;
use utils::input::*;
use utils::output::*;
use utils::results::*;

#[macro_use]
extern crate log;

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let day = parse_day(file!());
    let file = file_name_from_args();
    let contents: String = read_string_from_file(&path_for_day(day, &file).unwrap()).unwrap();
    let positions: Vec<u32> = contents
        .split(',')
        .map(|p| p.parse::<u32>())
        .try_collect()
        .unwrap();

    info!(
        "Solution to part one: {}",
        display_result(part_one(&positions))
    );
    info!(
        "Solution to part two: {}",
        display_result(part_two(&positions))
    );
}

fn part_one(positions: &[u32]) -> Result<u64> {
    let (min, max) = positions
        .iter()
        .minmax()
        .into_option()
        .ok_or_else(|| AOCError::new_from_ref("Empty list of positions"))?;
    let minmax = (*min..max + 1).map(|pos| fuel_for(positions, pos)).minmax();

    minmax
        .into_option()
        .map(|(min, _)| min)
        .ok_or_else(|| AOCError::new_from_ref("Empty list of positions"))
}

fn fuel_for(positions: &[u32], position: u32) -> u64 {
    let sum: i64 = positions
        .iter()
        .map(|&pos| (pos as i64 - position as i64).abs())
        .sum();
    sum as u64
}

fn incr_fuel_for(positions: &[u32], position: u32, current_min: u64) -> Option<u64> {
    let result: itertools::FoldWhile<u64> = positions.iter().fold_while(0, |sum, &pos| {
        let new_sum =
            (0..(position as i64 - pos as i64).abs() as u64).fold(sum, |sum, i| sum + i + 1);
        if new_sum >= current_min {
            itertools::FoldWhile::Done(new_sum)
        } else {
            itertools::FoldWhile::Continue(new_sum)
        }
    });

    Some(result.into_inner()).filter(|&new| new <= current_min)
}

fn part_two(positions: &[u32]) -> Result<u64> {
    let sorted: Vector<&u32> = positions.iter().sorted().collect();
    let start_position = *sorted[sorted.len() / 2];
    let candidates = (*sorted[0]..*sorted[sorted.len() - 1])
        .sorted_by_key(|&pos| (start_position as i64 - pos as i64).abs());

    let min = candidates.fold(std::u64::MAX, |min, pos| {
        incr_fuel_for(positions, pos, min).unwrap_or(min)
    });

    Ok(min)
}

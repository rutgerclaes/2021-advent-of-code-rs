use itertools::Itertools;
use utils::input::*;
use utils::output::*;
use utils::results::*;

fn main() {
    let day = parse_day(file!());
    let file = file_name_from_args();
    let depths: Vec<i32> =
        parse_lines_from_file(&utils::input::path_for_day(day, &file).unwrap()).unwrap();

    println!(
        "Solution to part one: {}",
        display_result(part_one(&depths))
    );
    println!(
        "Solution to part two: {}",
        display_result(part_two(&depths))
    );
}

fn part_one(depths: &[i32]) -> Result<usize> {
    Ok(depths.iter().tuple_windows().filter(|(a, b)| a < b).count())
}

fn part_two(depths: &[i32]) -> Result<usize> {
    Ok(depths
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count())
}

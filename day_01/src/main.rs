use itertools::Itertools;
use utils::input::*;
use utils::output::*;
use utils::results::*;

#[macro_use]
extern crate log;

fn main() {
    init_env_log();

    let day = parse_day(file!());
    let file = file_name_from_args();
    let depths: Vec<i32> =
        parse_lines_from_file(&utils::input::path_for_day(day, &file).unwrap()).unwrap();

    info!(
        "Solution to part one: {}",
        display_result(part_one(&depths))
    );
    info!(
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part_one_is_correct() {
        let depths = vec![1, 2, 3, 2, 1];
        assert_eq!(part_one(&depths), Ok(2));

        let depths = vec![9, 8, 7, 6];
        assert_eq!(part_one(&depths), Ok(0));

        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part_one(&depths), Ok(7));
    }

    #[test]
    fn part_two_is_correct() {
        let depths = vec![1, 2, 3];
        assert_eq!(part_two(&depths), Ok(0));

        let depths = vec![1, 2, 3, 4];
        assert_eq!(part_two(&depths), Ok(1));

        let depths = vec![1, 2, 3, 1];
        assert_eq!(part_two(&depths), Ok(0));

        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part_two(&depths), Ok(5));
    }
}

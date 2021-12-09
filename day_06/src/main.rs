use im_rc::Vector;
use itertools::Itertools;
use tailcall::tailcall;
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
    let fish: Vector<u8> = contents
        .split(",")
        .map(|p| p.parse::<u8>())
        .try_collect()
        .unwrap();

    info!("Solution to part one: {}", display_result(part_one(&fish)));
    info!("Solution to part two: {}", display_result(part_two(&fish)));
}

fn part_one(fish: &Vector<u8>) -> Result<usize> {
    Ok(simulate(fish.clone(), 80))
}

fn part_two(fish: &Vector<u8>) -> Result<usize> {
    let grouped = fish.iter().counts();
    let fish_counts: Vector<usize> = (0..9)
        .map(|i| grouped.get(&i).unwrap_or(&0))
        .copied()
        .collect();
    Ok(simulate_group(fish_counts, 256))
}

#[tailcall]
fn simulate(fish: Vector<u8>, iterations: usize) -> usize {
    #[tailcall]
    fn traverse(index: usize, fish: Vector<u8>, extra_fishes: usize) -> Vector<u8> {
        match fish.get(index) {
            Some(f) if *f == 0 => {
                let updated = fish.update(index, 6);
                traverse(index + 1, updated, extra_fishes + 1)
            }
            Some(f) => {
                let updated = fish.update(index, f - 1);
                traverse(index + 1, updated, extra_fishes)
            }
            None => {
                let mut result = fish.clone();
                result.append(std::iter::repeat(8).take(extra_fishes).collect());
                result
            }
        }
    }

    if iterations == 0 {
        fish.len()
    } else {
        let next_iter = traverse(0, fish.clone(), 0);
        simulate(next_iter, iterations - 1)
    }
}

#[tailcall]
fn simulate_group(mut fish_count: Vector<usize>, iterations: usize) -> usize {
    if iterations == 0 {
        fish_count.iter().sum()
    } else {
        let reproduce_count = fish_count.pop_front().unwrap();
        fish_count.push_back(reproduce_count);
        *fish_count.get_mut(6).unwrap() += reproduce_count;
        simulate_group(fish_count, iterations - 1)
    }
}

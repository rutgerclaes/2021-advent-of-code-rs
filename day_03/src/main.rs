use std::fmt::Display;
use std::str::FromStr;
use utils::input::*;
use utils::output::*;
use utils::results::*;

#[macro_use]
extern crate log;

fn main() {
    init_env_log();

    let day = parse_day(file!());
    let file = file_name_from_args();
    let numbers: Vec<BinaryNumber> =
        parse_lines_from_file(&path_for_day(day, &file).unwrap()).unwrap();

    info!(
        "Solution to part one: {}",
        display_result(part_one(&numbers))
    );
    info!(
        "Solution to part two: {}",
        display_result(part_two(&numbers))
    );
}

#[derive(Debug, Clone)]
struct BinaryNumber(Vec<bool>);

impl BinaryNumber {
    fn new(vec: Vec<bool>) -> BinaryNumber {
        BinaryNumber(vec)
    }

    fn bit_at(&self, index: usize) -> bool {
        self.0[index]
    }

    fn counts(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.0.iter().map(|&b| if b { (1, 0) } else { (0, 1) })
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Display for BinaryNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|&b| if b { '1' } else { '0' })
                .collect::<String>()
        )
    }
}

impl From<&BinaryNumber> for u32 {
    fn from(number: &BinaryNumber) -> Self {
        number
            .0
            .iter()
            .fold(0, |sum, &bit| (sum << 1) + if bit { 1 } else { 0 })
    }
}

impl FromStr for BinaryNumber {
    type Err = AOCError;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        Ok(BinaryNumber::new(input.chars().map(|c| c == '1').collect()))
    }
}

fn part_one(numbers: &[BinaryNumber]) -> Result<u32> {
    let mut iterator = numbers.iter();

    iterator
        .next()
        .map(|head| {
            let counts: Vec<(usize, usize)> = iterator.fold(head.counts().collect(), {
                |counts, number| {
                    counts
                        .iter()
                        .zip(number.counts())
                        .map(|((a, b), (c, d))| (a + c, b + d))
                        .collect()
                }
            });

            let gamma =
                BinaryNumber::new(counts.iter().map(|(ones, zeros)| ones > zeros).collect());
            debug!("Gamma value: {} ({})", gamma, u32::from(&gamma));
            let epsilon =
                BinaryNumber::new(counts.iter().map(|(ones, zeros)| ones < zeros).collect());
            debug!("Epsilon value: {} ({})", epsilon, u32::from(&epsilon));

            u32::from(&gamma) * u32::from(&epsilon)
        })
        .ok_or_else(|| AOCError::new_from_ref("Empty list of numbers"))
}

fn part_two(numbers: &[BinaryNumber]) -> Result<u32> {
    let oxygen_generator_rating = filter_by_bit(numbers.iter().collect(), 0, true)?;
    debug!(
        "Oxygen generator rating: {} ({})",
        oxygen_generator_rating,
        u32::from(oxygen_generator_rating)
    );
    let co2_scrubber_rating = filter_by_bit(numbers.iter().collect(), 0, false)?;
    debug!(
        "CO2 scrubber rating: {} ({})",
        co2_scrubber_rating,
        u32::from(co2_scrubber_rating)
    );

    Ok(u32::from(oxygen_generator_rating) * u32::from(co2_scrubber_rating))
}

#[allow(clippy::collapsible_else_if)]
fn filter_by_bit(
    list: Vec<&BinaryNumber>,
    index: usize,
    keep_largest: bool,
) -> Result<&BinaryNumber> {
    if list.is_empty() {
        Err(AOCError::new_from_ref("No number found"))
    } else if list.len() == 1 {
        Ok(list.get(0).unwrap())
    } else if list[0].len() <= index {
        Err(AOCError::new(format!(
            "Trying to partition by index {} on lists of length {}",
            index,
            list[0].len()
        )))
    } else {
        let (ones, zeros): (Vec<&BinaryNumber>, Vec<&BinaryNumber>) =
            list.iter().cloned().partition(|n| n.bit_at(index));

        if ones.len() >= zeros.len() {
            if keep_largest {
                filter_by_bit(ones, index + 1, keep_largest)
            } else {
                filter_by_bit(zeros, index + 1, keep_largest)
            }
        } else {
            if keep_largest {
                filter_by_bit(zeros, index + 1, keep_largest)
            } else {
                filter_by_bit(ones, index + 1, keep_largest)
            }
        }
    }
}

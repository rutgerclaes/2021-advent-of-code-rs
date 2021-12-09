use im_rc::HashSet;
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
    let notes: Vec<Note> = parse_lines_from_file(&path_for_day(day, &file).unwrap()).unwrap();

    info!("Solution to part one: {}", display_result(part_one(&notes)));
    info!("Solution to part two: {}", display_result(part_two(&notes)));
}

fn part_one(notes: &[Note]) -> Result<usize> {
    let result = notes
        .iter()
        .flat_map(|note| note.output_patterns.iter())
        .filter(|output| {
            let nb_of_signals = output.signals.len();
            nb_of_signals == 2 || nb_of_signals == 3 || nb_of_signals == 4 || nb_of_signals == 7
        })
        .count();

    Ok(result)
}

fn part_two(notes: &[Note]) -> Result<usize> {
    let outputs: Vec<usize> = notes.iter().map(|note| note.decode()).try_collect()?;
    Ok(outputs.iter().sum())
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Signal {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl FromStr for Signal {
    type Err = AOCError;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let ch = input
            .chars()
            .exactly_one()
            .map_err(|_| AOCError::new_from_ref("Input to Signal wasn't exactly one char"))?;
        match ch {
            'a' => Ok(Signal::A),
            'b' => Ok(Signal::B),
            'c' => Ok(Signal::C),
            'd' => Ok(Signal::D),
            'e' => Ok(Signal::E),
            'f' => Ok(Signal::F),
            'g' => Ok(Signal::G),
            _ => Err(AOCError::new_from_ref("Unsupported signal char")),
        }
    }
}

impl From<Signal> for char {
    fn from(signal: Signal) -> Self {
        match signal {
            Signal::A => 'a',
            Signal::B => 'b',
            Signal::C => 'c',
            Signal::D => 'd',
            Signal::E => 'e',
            Signal::F => 'f',
            Signal::G => 'g',
        }
    }
}

impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", char::from(*self))
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct SignalPattern {
    signals: HashSet<Signal>,
}

impl SignalPattern {
    fn new(signals: HashSet<Signal>) -> SignalPattern {
        SignalPattern { signals }
    }

    fn count_overlap_with(&self, other: &SignalPattern) -> usize {
        self.signals.iter().fold(0, |count, elem| {
            count + if other.signals.contains(elem) { 1 } else { 0 }
        })
    }

    fn is_equivalent_to(&self, other: &SignalPattern) -> bool {
        self.signals == other.signals
    }
}

impl Display for SignalPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            self.signals
                .iter()
                .map(|&signal| char::from(signal))
                .join("")
        )
    }
}

impl FromStr for SignalPattern {
    type Err = AOCError;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let signals: Vec<Signal> = input
            .chars()
            .map(|c| c.to_string().parse::<Signal>())
            .try_collect()?;
        Ok(SignalPattern::new(HashSet::from(signals)))
    }
}

#[derive(Debug)]
struct Note {
    digit_patterns: Vec<SignalPattern>,
    output_patterns: Vec<SignalPattern>,
}

impl Note {
    fn find_pattern_with_len(&self, length: usize) -> Result<&SignalPattern> {
        self.digit_patterns
            .iter()
            .filter(|d| d.signals.len() == length)
            .exactly_one()
            .map_err(|err| {
                AOCError::new(format!(
                    "Could not find unique pattern with length {}: {}",
                    length, err
                ))
            })
    }

    fn find_patterns_with_len(&self, length: usize) -> HashSet<&SignalPattern> {
        self.digit_patterns
            .iter()
            .filter(|d| d.signals.len() == length)
            .collect()
    }

    fn decode(&self) -> Result<usize> {
        let one: &SignalPattern = self.find_pattern_with_len(2)?;
        let four: &SignalPattern = self.find_pattern_with_len(4)?;
        let seven: &SignalPattern = self.find_pattern_with_len(3)?;
        let eight: &SignalPattern = self.find_pattern_with_len(7)?;

        fn find_by_overlap_with<'a>(
            possibilities: &HashSet<&'a SignalPattern>,
            reference: &SignalPattern,
            overlap: usize,
        ) -> Result<&'a SignalPattern> {
            possibilities
                .iter()
                .filter(|pattern| pattern.count_overlap_with(reference) == overlap)
                .copied()
                .exactly_one()
                .map_err(|e| {
                    AOCError::new(format!(
                        "Couldn't find pattern overlapping with {} {}: {}",
                        overlap, reference, e
                    ))
                })
        }

        fn find_last_one<'a>(
            possibilities: &HashSet<&'a SignalPattern>,
        ) -> Result<&'a SignalPattern> {
            possibilities
                .iter()
                .copied()
                .exactly_one()
                .map_err(|e| AOCError::new(e.to_string()))
        }

        let mut five_signals: HashSet<&SignalPattern> = self.find_patterns_with_len(5);

        let three: &SignalPattern = find_by_overlap_with(&five_signals, one, 2)?;
        five_signals.remove(three);

        let five: &SignalPattern = find_by_overlap_with(&five_signals, four, 3)?;
        five_signals.remove(five);

        let two: &SignalPattern = find_last_one(&five_signals)?;

        let mut six_signals: HashSet<&SignalPattern> = self.find_patterns_with_len(6);

        let nine: &SignalPattern = find_by_overlap_with(&six_signals, four, 4)?;
        six_signals.remove(nine);

        let zero: &SignalPattern = find_by_overlap_with(&six_signals, five, 4)?;
        six_signals.remove(zero);

        let six: &SignalPattern = find_last_one(&six_signals)?;
        let patterns = vec![zero, one, two, three, four, five, six, seven, eight, nine];

        let decode = |input: &SignalPattern| {
            patterns
                .iter()
                .enumerate()
                .find_map(|(value, pattern)| {
                    Some(value).filter(|_| pattern.is_equivalent_to(input))
                })
                .ok_or_else(||AOCError::new( format!("Output pattern {} not found", input ) ))
        };

        let digits: Vec<usize> = self.output_patterns.iter().map(decode).try_collect()?;
        let number: usize = digits.iter().fold(0, |number, digit| number * 10 + digit);
        Ok(number)
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "{} | {}",
            self.digit_patterns
                .iter()
                .map(|p| format!("{}", p))
                .join(" "),
            self.output_patterns
                .iter()
                .map(|p| format!("{}", p))
                .join(" ")
        )
    }
}

impl FromStr for Note {
    type Err = AOCError;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let (signal_string, output_string) = input
            .split(" | ")
            .collect_tuple()
            .ok_or_else(||AOCError::new_from_ref("Malformed input"))?;

        let digits = signal_string
            .split_whitespace()
            .map(|string| string.parse::<SignalPattern>())
            .try_collect()?;
        let outputs = output_string
            .split_whitespace()
            .map(|string| string.parse::<SignalPattern>())
            .try_collect()?;

        Ok(Note {
            digit_patterns: digits,
            output_patterns: outputs,
        })
    }
}

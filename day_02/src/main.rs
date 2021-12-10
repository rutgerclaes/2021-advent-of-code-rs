use itertools::Itertools;
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
    let instructions: Vec<Instruction> =
        parse_lines_from_file(&path_for_day(day, &file).unwrap()).unwrap();

    info!(
        "Solution to part one: {}",
        display_result(part_one(&instructions))
    );
    info!(
        "Solution to part two: {}",
        display_result(part_two(&instructions))
    );
}

fn part_one(instructions: &[Instruction]) -> Result<i64> {
    let result = instructions
        .iter()
        .fold(Position::zero(), |position, instruction| {
            position.apply(instruction)
        });
    Ok(result.horizontal as i64 * result.depth as i64)
}

fn part_two(instructions: &[Instruction]) -> Result<i64> {
    let result = instructions
        .iter()
        .fold(PositionAndAim::zero(), |position, instruction| {
            position.apply(instruction)
        });
    Ok(result.position.horizontal as i64 * result.position.depth as i64)
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    fn zero() -> Position {
        Position::new(0, 0)
    }

    fn new(horizontal: i32, depth: i32) -> Position {
        Position { horizontal, depth }
    }

    fn apply(&self, instruction: &Instruction) -> Position {
        match instruction.direction {
            Direction::Down => {
                Position::new(self.horizontal, self.depth + instruction.steps as i32)
            }
            Direction::Up => Position::new(self.horizontal, self.depth - instruction.steps as i32),
            Direction::Forward => {
                Position::new(self.horizontal + instruction.steps as i32, self.depth)
            }
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "horizontal: {:+05} depth: {:+05}",
            self.horizontal, self.depth
        )
    }
}

#[derive(Debug, PartialEq)]
struct PositionAndAim {
    aim: i32,
    position: Position,
}

impl PositionAndAim {
    fn zero() -> PositionAndAim {
        PositionAndAim::new(0, Position::zero())
    }

    fn new(aim: i32, position: Position) -> PositionAndAim {
        PositionAndAim { aim, position }
    }

    fn apply(&self, instruction: &Instruction) -> PositionAndAim {
        match instruction.direction {
            Direction::Down => {
                PositionAndAim::new(self.aim + instruction.steps as i32, self.position)
            }
            Direction::Up => {
                PositionAndAim::new(self.aim - instruction.steps as i32, self.position)
            }
            Direction::Forward => PositionAndAim::new(
                self.aim,
                Position::new(
                    self.position.horizontal + instruction.steps as i32,
                    self.position.depth + self.aim * instruction.steps as i32,
                ),
            ),
        }
    }
}

impl Display for PositionAndAim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "aim: {:+05} {}", self.aim, self.position)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Instruction {
    direction: Direction,
    steps: u32,
}

impl Instruction {
    fn new(direction: Direction, steps: u32) -> Instruction {
        Instruction { direction, steps }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Down,
    Up,
    Forward,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} {}", self.direction, self.steps)
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let output = match self {
            Direction::Down => "down",
            Direction::Up => "up",
            Direction::Forward => "forward",
        };
        write!(f, "{}", output)
    }
}

impl FromStr for Direction {
    type Err = AOCError;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match input {
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            "forward" => Ok(Direction::Forward),
            unrec => Err(AOCError::new(format!(
                "Failed to parse '{}' as direction",
                unrec
            ))),
        }
    }
}

impl FromStr for Instruction {
    type Err = AOCError;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        if let Some((direction_input, steps_input)) = input.split_whitespace().collect_tuple() {
            let direction = direction_input.parse()?;
            let steps = steps_input.parse::<u32>()?;
            Ok(Instruction::new(direction, steps))
        } else {
            Err(AOCError::new(format!("Could not parse '{}'", input)))
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn position_can_be_created() {
        let zero = Position::zero();
        assert_eq!(zero, Position::new(0, 0));
        assert_eq!(zero.horizontal, 0);
        assert_eq!(zero.depth, 0);
    }

    #[test]
    fn position_can_interpret_instruction() {
        let zero = Position::zero();

        let down = zero.apply(&Instruction::new(Direction::Down, 42));
        assert_eq!(down.depth, 42);
        assert_eq!(down.horizontal, 0);

        let up = down.apply(&Instruction::new(Direction::Up, 20));
        assert_eq!(up.depth, 22);
        assert_eq!(up.horizontal, 0);

        let forward = up.apply(&Instruction::new(Direction::Forward, 7));
        assert_eq!(forward.depth, 22);
        assert_eq!(forward.horizontal, 7);
    }

    #[test]
    fn position_and_aim_can_be_created() {
        let zero = PositionAndAim::zero();
        assert_eq!(zero, PositionAndAim::new(0, Position::new(0, 0)));
        assert_eq!(zero.aim, 0);
        assert_eq!(zero.position, Position::zero());
    }

    #[test]
    fn position_and_aim_can_interpret_instructions() {
        let zero = PositionAndAim::zero();

        let down = zero.apply(&Instruction::new(Direction::Down, 2));
        assert_eq!(down.position, zero.position);
        assert_eq!(down.aim, 2);

        let forward_down = down.apply(&Instruction::new(Direction::Forward, 2));
        assert_eq!(forward_down.position, Position::new(2, 4));
        assert_eq!(forward_down.aim, 2);

        let level = forward_down.apply(&Instruction::new(Direction::Up, 2));
        assert_eq!(level.position, forward_down.position);
        assert_eq!(level.aim, 0);

        let forward_level = level.apply(&Instruction::new(Direction::Forward, 3));
        assert_eq!(forward_level.position, Position::new(5, 4));
        assert_eq!(forward_level.aim, 0);

        let up = forward_level.apply(&Instruction::new(Direction::Up, 1));
        assert_eq!(up.position, forward_level.position);
        assert_eq!(up.aim, -1);

        let forward_up = up.apply(&Instruction::new(Direction::Forward, 4));
        assert_eq!(forward_up.position, Position::new(9, 0));
        assert_eq!(forward_up.aim, -1);
    }

    #[test]
    fn parse_instruction() {
        assert_eq!(
            "forward 5".parse::<Instruction>(),
            Ok(Instruction::new(Direction::Forward, 5))
        );
        assert_eq!(
            "down 2".parse::<Instruction>(),
            Ok(Instruction::new(Direction::Down, 2))
        );
        assert_eq!(
            "up 3".parse::<Instruction>(),
            Ok(Instruction::new(Direction::Up, 3))
        );
    }
}

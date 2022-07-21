use rand::Rng;
use regex::Regex;
use std::ops::Range;

pub fn roll(command: &str) -> Result<u32, String> {
    let res = match parse_command(command) {
        Ok(res) => res,
        Err(err) => {
            return Result::Err(err);
        }
    };

    let mut sum = 0;

    for _ in 0..res.number {
        sum += roll_dice(res.sides)
    }

    Result::Ok(sum)
}

struct DiceCommand {
    number: i32,
    sides: u32,
}

fn parse_command(command: &str) -> Result<DiceCommand, String> {
    let dice_roll_regex = Regex::new(r"^(?P<number>\d+) ?(?:d(?P<sides>\d+))$?").unwrap();

    let cap = match dice_roll_regex.captures(command) {
        Some(c) => c,
        None => {
            return Err(format!(
                "Unable to parse dice roll command, double check the syntax. `{:?}`",
                command
            ));
        }
    };

    let number_of_dice = cap
        .name("number")
        .map_or(1, |m| m.as_str().parse().unwrap());
    let dice_sides = cap.name("sides").map_or(6, |m| m.as_str().parse().unwrap());

    Result::Ok(DiceCommand {
        number: number_of_dice,
        sides: dice_sides,
    })
}

/// Roll a dice with a given number of sides.
fn roll_dice(sides: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(Range {
        start: 1,
        end: sides,
    })
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn rolls_within_range() {
        // Because the logic relies on random, run the test multiple times.
        for _ in 0..100 {
            let res = roll_dice(20);
            assert_eq!(false, res > 20);
            assert_eq!(false, res < 1);
        }
    }

    #[test]
    fn parse_invalid_roll() {
        let res = parse_command("1e20");
        assert_eq!(true, res.is_err());
    }

    #[test]
    fn parse_valid_roll() {
        let res = parse_command("1d20").unwrap();
        assert_eq!(1, res.number);
        assert_eq!(20, res.sides);
    }

    #[test]
    fn parse_valid_roll_with_space() {
        let res = parse_command("1 d20").unwrap();
        assert_eq!(1, res.number);
        assert_eq!(20, res.sides);
    }
}

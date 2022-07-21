use rand::Rng;
use regex::Regex;
use std::ops::Range;

pub fn roll(command: &str) -> Result<u32, String> {
    let dice_roll_regex = Regex::new(r"(?P<number>\d+)(?:d(?P<sides>\d+))?").unwrap();

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

    let mut sum = 0;

    for _ in 0..number_of_dice {
        sum += roll_dice(dice_sides)
    }

    Result::Ok(sum)
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
    use crate::roll_dice;

    #[test]
    fn rolls_within_range() {
        // Because the logic relies on random, run the test multiple times.
        for _ in 0..100 {
            let res = roll_dice(20);
            assert_eq!(false, res > 20);
            assert_eq!(false, res < 1);
        }
    }
}

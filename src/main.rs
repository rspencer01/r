extern crate getrandom;

use std::{env, fmt};

const USAGE_STRING: &str = "Usage:
  r <dice>

Valid Dice:
  All valid dice strings are of the form
      XdY+Z
    or
      XdY-Z
  which indicates one should roll X dice with Y sides and add or subtract Z
  from the result.

Example:
  r 2d8+4
";

#[derive(Debug)]
struct InvalidDie;
impl fmt::Display for InvalidDie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid die")
    }
}
impl std::error::Error for InvalidDie {}

#[derive(Copy, Clone)]
struct DiceRoll {
    count: u16,
    sides: u16,
    modify: i16,
}

fn random(upper_bound: u16) -> u16 {
    let mx = (u16::MAX - upper_bound).next_multiple_of(upper_bound);
    let mut r: u16;
    let mut x = [0; 2];
    loop {
        getrandom::getrandom(&mut x).expect("Error obtaining random bits");
        r = u16::from_be_bytes(x);
        if r < mx {
            break;
        }
    }
    r % upper_bound
}

impl DiceRoll {
    fn from(desc: &str) -> Result<DiceRoll, InvalidDie> {
        let mut modify = 0;
        let mut die = desc;
        if let Some(m_idx) = desc.find(|c| c == '+' || c == '-') {
            modify = desc[m_idx..].parse().map_err(|_| InvalidDie)?;
            die = &desc[..m_idx];
        };
        let d_index = die.find('d').ok_or(InvalidDie)?;
        let count = die[..d_index].parse().map_err(|_| InvalidDie)?;
        let sides = die[d_index + 1..].parse().map_err(|_| InvalidDie)?;
        Ok(DiceRoll {
            count,
            sides,
            modify,
        })
    }

    fn roll(&self) -> i16 {
        std::iter::from_fn(|| Some(1 + random(self.sides)))
            .take(self.count as usize)
            .sum::<u16>() as i16
            + self.modify
    }
}

impl fmt::Display for DiceRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modify != 0 {
            write!(f, "{}d{}{:+}", self.count, self.sides, self.modify)
        } else {
            write!(f, "{}d{}", self.count, self.sides)
        }
    }
}

fn main() {
    let [_, dice_str] = env::args().collect::<Vec<_>>().leak() else {
        return println!("{}", USAGE_STRING);
    };
    match DiceRoll::from(dice_str) {
        Ok(die) => {
            let roll = die.roll();
            println!("{die} : {roll}");
        }
        Err(e) => {
            println!("{e}");
        }
    }
}

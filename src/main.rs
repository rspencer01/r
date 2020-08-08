extern crate rand;

use std::env;
use std::fmt;

use crate::rand::Rng;

#[derive(Copy, Clone, Debug)]
struct DieRoll {
    count: u16,
    sides: u16,
    modify: i16,
}

impl DieRoll {
    fn from(desc : &str) -> Result<DieRoll, &str> {
        let count;
        let sides;
        let modify;
        let die;
        match desc.find(|c| c == '+' || c == '-') {
            Some(i) => {
                modify = match desc[i..].parse() {
                    Ok(x) => x,
                    Err(_) => return Err("Not a valid die"),
                };
                die = &desc[..i];
            }
            None => {
                modify = 0;
                die = desc;
            }
        };
        match die.find('d') {
            Some(i) => {
                count = match die[..i].parse::<u16>() {
                    Ok(x) => x,
                    Err(_) => return Err("Not a valid die"),
                };
                sides = match die[i+1..].parse::<u16>() {
                    Ok(x) => x,
                    Err(_) => return Err("Not a valid die"),
                };
            }
            None => {
                return Err("Not a valid die");
            }
        }
        Ok(DieRoll {
            count,
            sides,
            modify,
        })
    }

    fn roll(&self) -> i16 {
        let mut rng = rand::thread_rng();
        (0..self.count).map(|_| {rng.gen_range(1, self.sides+1)}).sum::<u16>() as i16 + self.modify
    }
}

impl fmt::Display for DieRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modify > 0 {
            write!(f, "{}d{}+{}", self.count, self.sides, self.modify)
        } else if self.modify < 0 {
            write!(f, "{}d{}{}", self.count, self.sides, self.modify)
        } else {
            write!(f, "{}d{}", self.count, self.sides)
        }
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:");
        println!("  r <dice>");
        println!();
        println!("Exmple:");
        println!("  r 2d8+4");
    }
    match DieRoll::from(&args[1]) {
        Ok(die) => {
            println!("{} : {}", die, die.roll());
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

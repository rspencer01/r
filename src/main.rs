extern crate rand;

use std::env;
use std::fmt;

use crate::rand::Rng;

#[derive(Copy, Clone, Debug)]
struct DieRoll {
    count: u16,
    sides: u16,
    modify: u16,
}

impl DieRoll {
    fn from(desc : &str) -> Result<DieRoll, &str> {
        let die_mod : Vec<&str> = desc.split("+").collect();
        let die;
        let modify;
        match die_mod.len() {
            1 => {
                modify = 0;
                die = desc;
            }
            2 => {
                modify = match die_mod[1].parse::<u16>() {
                    Ok(x) => x,
                    Err(_) => return Err("Not a valid die"),
                };
                die = die_mod[0];
            }
            _ => {
                return Err("Not a valid die")
            }
        }
        let die_count : Vec<&str> = die.split("d").collect();
        let count;
        let sides;
        match die_count.len() {
            2 => {
                count = match die_count[0].parse::<u16>() {
                    Ok(x) => x,
                    Err(_) => return Err("Not a valid die"),
                };
                sides = match die_count[1].parse::<u16>() {
                    Ok(x) => x,
                    Err(_) => return Err("Not a valid die"),
                };
                if sides == 0 || count == 0 {
                    return Err("Not a valid die");
                }
            }
            _ => {
                return Err("Not a valid die")
            }
        }
        Ok(DieRoll {
            count,
            sides,
            modify,
        })
    }

    fn roll(&self) -> u16 {
        let mut rng = rand::thread_rng();
        (0..self.count).map(|_| {rng.gen_range(1, self.sides+1)}).sum::<u16>() + self.modify
    }
}

impl fmt::Display for DieRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modify != 0 {
            write!(f, "{}d{}+{}", self.count, self.sides, self.modify)
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

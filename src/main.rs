extern crate getrandom;

use std::env;
use std::fmt;
use std::ops::Range;


#[derive(Copy, Clone, Debug)]
struct DieRoll {
    count: u16,
    sides: u16,
    modify: i16,
}

fn random_in(within : Range<u16>) -> u16 {
    let dx = within.end - within.start;
    let mx = (0xff_ff / dx) * dx;
    let mut r : u16;
    loop {
        let mut x = [0; 2];
        getrandom::getrandom(&mut x).expect("Error obtaining random bits");
        r = (x[0] as u16) << 8 | x[1] as u16;
        if r < mx {
            break;
        }
    }
    within.start + r % dx
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
        (0..self.count).map(|_| {random_in(1..self.sides+1)}).sum::<u16>() as i16 + self.modify
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

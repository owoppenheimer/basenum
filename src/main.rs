use libm::pow;
use argh::FromArgs;
use std::process::exit;

#[derive(Debug, FromArgs)]
/// A simple program that converts a number into specific number system.
struct Arguments {
    /// specify a number system (up to 36)
    #[argh(option, short = 'b', default = "2")]
    number_system: u64,

    /// convert into decimal
    #[argh(switch, short = 'd')]
    into_dec: bool,

    #[argh(positional)]
    number: String
}

// convert a number
fn to_base(mut num: u64, number_system: u64) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    while num > 0 {
        let remainder = (num % number_system) as usize;
        result.push(digits.chars().nth(remainder).unwrap());
        num /= number_system;
    }
    result.chars().rev().collect()
}

fn into_decimal(number: String, number_system: &u32) -> u64 {
    let mut result: f64 = 0.0;
    for (key, value) in number.chars().rev().enumerate() {
        result += value.to_digit(*number_system).unwrap() as f64 * pow(*number_system as f64, key as f64);
    }

    result as u64
}

fn main() {
    let args: Arguments = argh::from_env();
    if args.number_system > 36 {
        eprintln!("Number system cannot be higher than 36. ({})", args.number_system);
        exit(1);
    }

    if args.into_dec {
        println!("{}", into_decimal(args.number.to_string(), &(args.number_system as u32)));
    } else {
        let u64num = args.number.trim().parse::<u64>().expect(&format!("{}: Couldn't parse a number (u64)", args.number));

        if args.number_system == 2 {
            println!("{u64num:b}");
        } else {
            println!("{}", to_base(u64num, args.number_system));
        }
    }
}

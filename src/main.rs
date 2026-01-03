use argh::FromArgs;

#[derive(Debug, FromArgs)]
/// A simple program that converts a number into specific number system.
struct Arguments {
    /// specify a number system.
    #[argh(option, short = 'b')]
    number_system: Option<u64>,

    #[argh(positional)]
    number: u64
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

fn main() {
    let number_system: Arguments = argh::from_env();

    match number_system.number_system {
        Some(2) | std::option::Option::None => println!("{:b}", number_system.number),

        Some(value) => {
            println!("{}", to_base(number_system.number, value));
        },
    }
}

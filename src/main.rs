use std::io::Write;

use clap::Parser;
mod dice;
use crate::dice::dice::Dice;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long, short)]
    dice: Option<String>,
    #[arg(long, short, default_value_t = 0)]
    output: u8,
}

fn post(expression: &str, message: &str, mode: u8) {
    match mode {
        0 => println!("{}", message),                     // minimalist
        1 => println!("{} -> {:?}", expression, message), // extended
        _ => panic!("Invalid output mode"),
    }
}

fn main() {
    let args = Cli::parse();

    match args.dice {
        Some(def) => {
            let dice = Dice::new(&def);
            post(&def, &format!("{:?}", dice.roll()), args.output);
        }
        None => loop {
            print!("> ");
            std::io::stdout().flush().expect("");

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("erro na leitura de input");

            input = input.trim().to_string();

            if input == "close" || input == "exit" || input == "quit" {
                break;
            }

            let dice = Dice::new(&input);
            post(&dice.def, &format!("{:?}", dice.roll()), args.output);
        },
    }
}

use std::io::Write;

use clap::Parser;
use lazy_static::lazy_static;
mod dice;
mod funcs;
use crate::dice::dice::Dice;
use crate::funcs::funcs::*;
use regex::Regex;
use ron::from_str;

#[derive(Parser, Debug)]
struct Cli {
    dice: Option<String>,
    #[arg(long, short, default_value_t = 0)]
    output: u8,
}

fn post(expression: &str, message: &str, mode: u8) {
    match mode {
        0 => println!("{}", message),                   // minimalist
        1 => println!("{} -> {}", expression, message), // extended
        _ => panic!("Invalid output mode"),
    }
}

fn parser(expression: &str) -> String {
    lazy_static! {
        static ref REG_DICE: Regex = Regex::new(r"\d*d\d+").unwrap();
        static ref REG_FUNC: Regex = Regex::new(r"[a-zA-Z]{3}\(.*\)").unwrap();
        static ref REG_FUNC_ARG: Regex = Regex::new(r"\[.*\]").unwrap();
    }

    let mut expanded = expression.clone().to_string();

    for cap in REG_DICE.captures_iter(expression) {
        let def_dice = cap.get(0).unwrap().as_str();
        let rolled = format!("{:?}", Dice::new(def_dice).roll());
        expanded = expanded.replace(def_dice, &rolled);
    }

    let mut resolved = expanded.clone().to_string();

    for cap in REG_FUNC.captures_iter(&expanded) {
        let def_func = cap.get(0).unwrap().as_str();
        let def_arg = REG_FUNC_ARG
            .captures(def_func)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str();
        let rolled: Vec<i32> = from_str(def_arg).unwrap();
        let res = match &def_func[0..3] {
            "sum" => sum(&rolled),
            "max" => max(&rolled),
            "min" => min(&rolled),
            _ => panic!("Invalid function"),
        };

        resolved = resolved.replace(def_func, &format!("{}", res));
    }

    resolved
}

fn main() {
    let args = Cli::parse();

    match args.dice {
        Some(def) => {
            post(&def, &parser(&def), args.output);
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

            let res = parser(&input);
            post(&input, &res, args.output);
        },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn functions() {
        use crate::funcs::funcs::*;

        let example = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(sum(&example), 21);
        assert_eq!(max(&example), 6);
        assert_eq!(min(&example), 1);
    }
}

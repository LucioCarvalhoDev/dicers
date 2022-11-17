use std::io::Write;

use clap::Parser;
mod dice;
mod funcs;
use crate::dice::dice::Dice;
use crate::funcs::funcs::*;
use eval::eval;
use regex::Regex;
use ron::from_str;

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

fn parser(expression: &str) {
    let reg_dice = Regex::new(r"\d*d\d+").unwrap();
    let reg_func = Regex::new(r"[a-zA-Z]{3}\(.*\)").unwrap();
    let reg_func_arg = Regex::new(r"\[.*\]").unwrap();

    let mut expanded = expression.clone().to_string();

    for cap in reg_dice.captures_iter(expression) {
        let def_dice = cap.get(0).unwrap().as_str();
        let rolled = format!("{:?}", Dice::new(def_dice).roll());
        expanded = expanded.replace(def_dice, &rolled);
    }

    let mut resolved = expanded.clone().to_string();

    for cap in reg_func.captures_iter(&expanded) {
        let def_func = cap.get(0).unwrap().as_str();
        let def_arg = reg_func_arg
            .captures(def_func)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str();
        let rolled: Vec<i32> = from_str(def_arg).unwrap();
        let res = match &def_func[0..3] {
            "sum" => sum(&rolled),
            "max" => max(&rolled),
            _ => panic!("Invalid function"),
        };

        // println!("{} | {}", resolved, def_func);
        resolved = resolved.replace(def_func, &format!("{}", res));
    }

    // todo - finalizar parse

    println!("{}", evaluated);
}

fn main() {
    let args = Cli::parse();

    // match args.dice {
    //     Some(def) => {
    //         let dice = Dice::new(&def);
    //         post(&def, &format!("{:?}", dice.roll()), args.output);
    //     }
    //     None => loop {
    //         print!("> ");
    //         std::io::stdout().flush().expect("");

    //         let mut input = String::new();
    //         std::io::stdin()
    //             .read_line(&mut input)
    //             .expect("erro na leitura de input");

    //         input = input.trim().to_string();

    //         if input == "close" || input == "exit" || input == "quit" {
    //             break;
    //         }

    //         let dice = Dice::new(&input);
    //         post(&dice.def, &format!("{:?}", dice.roll()), args.output);
    //     },
    // }
    parser("max(2d6) + 6");
}

#[cfg(test)]
mod tests {
    use crate::parser;

    #[test]
    fn functions() {
        // use crate::funcs::funcs::*;

        // // let example = vec![1, 2, 3, 4, 5, 6];
        // // assert_eq!(sum(&example), 21);
        // // assert_eq!(max(&example), 6);
        // // assert_eq!(min(&example), 1);
        // // assert_eq!(med(&example), 3.5);
    }

    #[test]
    fn paser() {
        parser("max(2d6)");
    }
}

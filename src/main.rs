use clap::Parser;
mod dice;
use crate::dice::dice::Dice;

#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value = "1d20")]
    dice: String,
}

fn main() {
    let args = Args::parse();

    let d6 = Dice::new(&args.dice);

    println!("{} -> {:?}", d6.def, d6.roll());
}

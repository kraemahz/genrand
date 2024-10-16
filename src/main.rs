use clap::{Parser, ValueEnum};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum Appearance {
    #[default]
    Mixed,
    Uppercase,
    Lowercase,
}

#[derive(Parser, Debug)]
#[clap(name = "genrand", version = "0.1", author = "Teague Lasser")]
struct Args {
    #[clap(short, long, default_value_t = 7, required = false)]
    length: usize,
    #[clap(short, long, value_enum, required = false, default_value = "mixed")]
    appearance: Appearance,
}

fn main() {
    let args = Args::parse();
    let size = args.length;

    let mut rng = thread_rng();
    let chars: String = (0..size)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    let chars = match args.appearance {
        Appearance::Mixed => chars,
        Appearance::Uppercase => chars.to_uppercase(),
        Appearance::Lowercase => chars.to_lowercase(),
    };
    println!("{}", chars);
}

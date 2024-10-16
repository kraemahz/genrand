use clap::{Parser, ValueEnum};

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
    #[clap(short, long, default_value_t = 7)]
    length: usize,
    #[clap(short, long, value_enum, default_value = "mixed")]
    appearance: Appearance,
}

fn main() {
    use rand_core::{OsRng, RngCore};

    let args: Args = Args::parse();
        let chars: String = (0..args.length)
    .map(|_| {
        let idx: u8 = (OsRng.next_u32() % 62) as u8;
        match idx {
            0..=9 => (b'0' + idx) as char,
            10..=35 => (b'A' + (idx - 10)) as char,
            _ => (b'a' + (idx - 36)) as char,
        }
    })
    .collect();

    let chars: String = if matches!(args.appearance, Appearance::Uppercase) {
        chars.to_uppercase()
    } else if matches!(args.appearance, Appearance::Lowercase) {
        chars.to_lowercase()
    } else {
        chars
    };

    println!("{chars}");
}

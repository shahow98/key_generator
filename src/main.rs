use rand::{thread_rng, Rng};
use clap::Parser;

#[derive(Clone, Copy)]
enum GenMode {
    EASY,
    MEDIUM,
    HARD,
}

impl From<String> for GenMode {
    fn from(gen_mode: String) -> Self {
        let gen_mode = gen_mode.to_uppercase();
        match gen_mode {
            gen_mode if gen_mode == "EASY" => GenMode::EASY,
            gen_mode if gen_mode == "MEDIUM" => GenMode::MEDIUM,
            gen_mode if gen_mode == "HARD" => GenMode::HARD,
            _ => GenMode::HARD
        }
    }
}

/// Key Generator
#[derive(Parser, Debug)]
#[clap(author = "shahow", version = "1.0.0", about = "Key Generator", long_about = "Simple program to Key Generator")]
struct Args {
    /// Generate mode(EASY|MEDIUM|HARD)
    #[clap(short, long, default_value = "HARD")]
    mode: String,

    /// Key length
    #[clap(short, long, default_value_t = 10)]
    len: u8,
}

fn main() {
    let args: Args = Args::parse();
    let gen_mode: GenMode = args.mode.to_string().into();
    let len: u8 = args.len;
    for _ in 0..3 {
        let key = key_generate(init_char_dict(gen_mode), len);
        println!("{}", key);
    }
}

fn init_char_dict(mode: GenMode) -> Vec<String> {
    let mut dict = Vec::new();
    let num_dict = String::from("0123456789");
    let char_dict = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let symbol_dict = String::from("~!@#$%^&*()_+");

    match mode {
        GenMode::EASY => {
            dict.push(num_dict);
        }
        GenMode::MEDIUM => {
            dict.push(num_dict);
            dict.push(char_dict);
        }
        GenMode::HARD => {
            dict.push(num_dict);
            dict.push(char_dict);
            dict.push(symbol_dict);
        }
    }

    dict
}

fn key_generate(char_dict: Vec<String>, len: u8) -> String {
    let mut key = String::from("");
    for _ in 0..len {
        let idx = rand(char_dict.len());
        let charset: String = char_dict.get(idx).expect("Array index overflow").to_string();

        let idx = rand(charset.len());
        let value = charset.chars().nth(idx).expect("Char index overflow");
        key.push(value);
    }

    key
}

fn rand(max: usize) -> usize {
    let mut rand = thread_rng();
    let val: usize = rand.gen();
    val % max
}
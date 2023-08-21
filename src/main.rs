use clap::Parser;
use rand::Rng;


/// Generates a dashword (a word followed by a dash and a number) of arbitrary length.
/// By default it generates an 8 character dasshword composed of a 5 letter word, followed by a
/// dash and 2 digit number.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Length of the word
    #[arg(short, long, default_value_t = 5)]
    length: usize,

    /// Number of digits after the dash
    #[arg(short, long, default_value_t = 2)]
    digits: usize,
}

fn main() {

    let args = Args::parse();

    let len = args.length;
    let digits = args.digits;
    
    if len > 15 {
        eprintln!("unable to generate words longer than 15 characters");
        std::process::exit(1);
    }

    let mut rng = rand::thread_rng();

    let mut number = String::from("");

    for _ in 0..digits {
       let r = rng.gen_range(0..9); 
       number.push_str(&r.to_string());
    }

    let word = random_word::gen_len(len, random_word::Lang::En).unwrap();



    println!("{}-{}", word, number);
}

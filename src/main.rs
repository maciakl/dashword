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

    /// Use a word from a curated list of simple, memorable words instead of full dictionary.
    #[arg(short, long)]
    simple: bool
}

fn main() {

    let args = Args::parse();

    let len = args.length;
    let digits = args.digits;
    let simple = args.simple;

    let wordtype = if simple {"simple"} else {"dictionary"};
    
    let dashword = match get_dashword(len, digits, simple){
        Some(w) => w,
        None => {
            eprintln!("unable to generate {} words of length {} with {} digits", wordtype, len, digits);
            std::process::exit(1);
        }
    };

    println!("{}", dashword);
}





fn get_dashword(len:usize, digits:usize, simple:bool) -> Option<String> {
    
    if len <2 || len > 15 || digits < 1 || digits > 10 {
        return None;
    }

    let mut number:String = match get_number(digits) {
        Some(n) => n,
        None => {
            return None;
        }
    };

    let banned_numbers = vec![
        "14", "18", "23", "88", "69", "111", "311", "666", "1352", "1390", "1488",
    ];

    // make sure number is not one of the banned ones
    while banned_numbers.contains(&number.as_str()) {
        number = get_number(digits).unwrap();
    }

    let word:String;

    if simple {
        word = match get_curated_word(len) {
            Some(w) => w,
            None => {
                return None;
            }
        };
    }
    else {
        word = random_word::gen_len(len, random_word::Lang::En).unwrap().to_string();
    }

    Some(format!("{}-{}", word, number))
}





fn get_number(digits:usize) -> Option<String> {

    if digits < 1 || digits > 10 { return None; } 

    let mut rng = rand::thread_rng();
    let mut number = String::from("");

    for _ in 0..digits {
       let r = rng.gen_range(0..9); 
       number.push_str(&r.to_string());
    }

    return Some(number);
}






fn get_curated_word(len:usize) -> Option<String> {

    if len < 2 || len > 9 {
        return None;
    }
    let words = vec![
        // greek letters
        "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "iota", "kappa", "lambda", "omicron", "sigma",
        "tau", "omega", 
        // American military phonetic alphabet
        "bravo", "charlie", "foxtrot", "golf", "hotel", "juliet", "kilo", "lima", "mike", "november",
        "oscar", "papa", "quebec", "romeo", "sierra", "tango", "uniform", "victor", "whiskey", "xray",
        "yankee", "zulu",
        // months
        "january", "february", "march", "april", "may", "june", "july", "august", "september", "october",
        "december",
        // numbers
        "one", "two", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "zero",
        "first", "second", "third", "fifth", "seventh", "ninth", "eleventh", "double", "triple", "half",
        // directions
        "down", "left", "right", "center", "middle", "north", "south", "east", "west", "front", "back", 
        "forward", "above", "below", "top", "bottom", "beneath", "center",
        // celestial objects
        "io", "europa", "titan", "triton", "mars", "neptune", "mercury", "saturn", "jupiter", "pluto",
        // gems
        "amethyst", " pearl", "diamond", "emerald", "topaz", "opal", "ruby", "jade", "amber", "onyx", 
        "quartz", "jasper",
        // common words
        "time", "number", "people", "day", "part", "sound", "work", "year", "back", "name",
        "line", "farm", "land", "home", "hand", "air", "animal", "house", "page", "point",
        "answer", "school", "city", "eye", "story", "example", "paper", "side", "car", "night",
        "sea", "river", "idea", "face", "rust", "other", "many", "some", "new", "right", "great",
        "same", "another", "big", "even", "odd", "every", "next", "important", "above", "when", "then",
        "how", "up", "down", "no", "yes", "only", "very", "just", "around", "before", "after", "near",
        "far", "together", "often", "last", "first", "second", "third", "fourth", "fifth", "seventh",
        "tenth", "almost", "really", "sometimes", "soon", "ultra", "mega", "giga", "about", "alert",
        "audio", "aviod", "aware", "apart", "apple", "basic", "cover", "crash", "claim", "clean", "clear",
        "chair", "table", "click", "clock", "close", "open", "count", "entry", "group", "equal", "error",
        "event", "found", "guess", "guest", "every", "all", "exact", "extra", "input", "output", "media", 
        "minus", "plus", "month", "label", "ocean", "other", "major", "minor", "music", "radio", "round",
        "phone", "price", "place", "river", "quick", "slow", "sharp", "spare", "shift", "total", "sport",
        "tower", "small", "big", "large", "heavy", "stone", "until", "watch", "water", "value", "visit",
        "best", "exit", "cold", "city", "job", "buy", "old", "new", "set", "around", "access", "accept",
        "attend", "circle", "square", "common", "ability", "account", "display", "compare", "concept",
        "dynamic", "connect", "explore", "image", "install", "instead", "neutral", "minimum", "maximum", 
        "monitor", "partial", "pattern", "picture", "success", "support", "storage", "unknown", "upgrade",
        "abstract", "absolute", "academic", "computer", "concrete", "accuracy" ,"constant", "activity",
        "actually", "advanced", "category", "aircraft", "analysis", "anything", "apppendix", "assembly",
        "discount", "entrance", "distance", "district", "external", "document", "exchange", "excel", 
        "exciting", "history", "instance", "indirect", "keyboard", "interval", "maximize", "minimize",
        "positive", "negative", "official", "research", "security", "ultimate", "terminal", "variable",
        "vertical", "wireless",
    ];

    let tmp:Vec<_>= words.iter().filter(|w| w.len()==len).collect();

    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..tmp.len()-1);
    return Some(tmp[n].to_string());
}

#[cfg(test)]
mod tests;

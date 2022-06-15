use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("0");
        std::process::exit(0);
    }
    let word: String = args[1].clone();
    let mut sum: u32 = 0;

    for c in word.chars() {
        let i = c as u32;
        if i > 122 || i < 97 {
            println!("Invalid char: {}", c);
            std::process::exit(1);
        }
        sum += i - 96;
    }

    println!("{}", sum);
}
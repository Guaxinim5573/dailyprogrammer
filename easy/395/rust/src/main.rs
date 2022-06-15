use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let mut output: Vec<i32> = Vec::new();

    let mut streak = 0;
    for value in args {
        match value.as_str() {
            "1" => {
                streak += 1;
            },
            "0" => {
                if streak > 0 {
                    output.push(streak);
                }
                streak = 0;
            },
            _ => {
                println!("Invalid input: {}", value);
                std::process::exit(1);
            }
        }
    }
    if streak > 0 {
        output.push(streak);
    }
    println!("{:?}", output);
}

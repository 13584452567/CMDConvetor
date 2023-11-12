use std::process;
use std::env;
use std::i64;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("invalid input {}",args.len());
        process::exit(1);
    }
    let func = &args[1];
    let input = &args[2];
    if func == "dec2hex" {
        println!("{:x}", input.parse::<i32>().unwrap());
    }
    if func == "hex2dec" {
        let without_prefix = input.trim_start_matches("0x");
        let z = i64::from_str_radix(without_prefix, 16);
        println!("{:?}", z.unwrap());
    }
}
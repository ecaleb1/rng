#![allow(non_snake_case)]
//Usage: RNG [min] [max] [number of rolls]

use rand::Rng;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: RNG [min] [max] [number of rolls]");
        exit(0);
    }

    let min = args[1].trim().parse::<i32>().unwrap();
    let max = args[2].trim().parse::<i32>().unwrap();
    let mut n = 1;
    if args.len() > 3 {
        n = args[3].trim().parse::<i32>().unwrap();
    }


    //dbg!(min, max, n);
    let mut randomNumber;

    for _ in 0..n {
        randomNumber = rand::thread_rng().gen_range(min..=max);
        println!("{}", randomNumber);
    }
}

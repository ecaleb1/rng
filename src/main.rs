#![allow(non_snake_case)]
//Usage: rng [min] [max] [number of rolls]

use rand::Rng;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: rng [min] [max] [number of rolls]");
        exit(1);
    }

    let min = match args[1].trim().parse::<i32>() {
        Ok(min) => min,
        Err(_) => {println!("Error: minimum must be a number"); exit(1)}
    };
    let max = match args[2].trim().parse::<i32>() {
        Ok(max) => max,
        Err(_) => {println!("Error: maximum must be a number"); exit(1)}
    };
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

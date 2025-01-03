use clap::Parser;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    day: usize,

    #[arg(short, long)]
    part: usize,
}

fn main() {
    let args = Args::parse();

    match (args.day, args.part) {
        (1, 1) => println!("{}", day01::part_one()),
        (1, 2) => println!("{}", day01::part_two()),
        (2, 1) => println!("{}", day02::part_one()),
        (2, 2) => println!("{}", day02::part_two()),
        (3, 1) => println!("{}", day03::part_one()),
        (3, 2) => println!("{}", day03::part_two()),
        (4, 1) => println!("{}", day04::part_one()),
        (4, 2) => println!("{}", day04::part_two()),
        (5, 1) => println!("{}", day05::part_one()),
        (5, 2) => println!("{}", day05::part_two()),
        (day, part) => {
            eprintln!(
                "error: unsupported day and part combination: {} {}",
                day, part
            );
            std::process::exit(1);
        }
    }
}

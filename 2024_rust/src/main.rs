use clap::Parser;

mod day01;

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
        (day, part) => {
            eprintln!(
                "error: unsupported day and part combination: {} {}",
                day, part
            );
            std::process::exit(1);
        }
    }
}

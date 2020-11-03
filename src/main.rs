use clap::Clap;

#[derive(Clap, Debug)]
#[clap(version = "3.0.0-beta.2")]
struct Options {
    start: usize,
    end: usize,
    number: usize,
}

fn main() {
    let options = Options::parse();

    println!("{:?}", options);
}

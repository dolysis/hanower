use clap::Clap;

#[derive(Clap, Debug)]
#[clap(version = "3.0.0-beta.2")]
struct Options {
    #[clap(short, long, default_value = "1.0")]
    start: usize,

    #[clap(short, long, default_value = "2.0")]
    end: usize,

    #[clap(short, long, default_value = "2")]
    number: usize,
}

fn exponential_progression(a: f64, b: f64, c: usize) -> Vec<f64> {
    if c == 2 {
        println!("{:?}", vec![a, b]); // remove
        return vec![a, b];
    } else if c > 2 {
        let steps = 1.0;
        println!("{:?}", vec![a, steps, b]); // remove
        return vec![a, steps, b];
    }

    let placeholder_return = vec![1.0];
    return placeholder_return;
}

fn main() {
    // let options = Options::parse();
    // println!("{:?}", options);

    exponential_progression(1.0, 2.0, 3);
}

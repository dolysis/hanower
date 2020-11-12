use csv;
// use std::collections::BinaryHeap;
use std::error::Error;
use std::mem;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "hanoi")]
struct Opts {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(default_value = "10.0")]
    start: f64,

    #[structopt(default_value = "100.0")]
    end: f64,

    #[structopt(default_value = "2")]
    number: usize,

    #[structopt(short, long)]
    file: String,
}

fn find_fenceposts(mut start: f64, mut end: f64, gaps: usize) -> Vec<f64> {
    // swaps start and end values if they are incorrectly sized
    if start > end {
        mem::swap(&mut start, &mut end);
        println!("Start and end values swapped.");
    }

    if gaps == 2 {
        return vec![start, end];
    } else if gaps > 2 {
        // scale end value down according to start value
        // start must always move down to 1.0
        // let scaled_end = end - start + 1.0;

        let increment = (end - start + 1.0).ln() / gaps as f64;
        let mut return_vec = vec![0.0];
        let mut increment_value = 0.0;
        // iterate over desired length of return vector (== gaps)
        // fill in the incremental fencepost values
        for _n in 0..=gaps {
            increment_value = increment_value + increment;
            return_vec.push(increment_value);
        }

        return return_vec;
    } else {
        println!("Gaps too small.");
        return vec![start, end];
    }
}

// uses find_fenceposts() vec to identify buckets
// and then pulls the index number & row data from the max index
// in those buckets
fn bucket_maxes(fence_vec: Vec<f64>, file: String) -> Vec<f64> {
    let _min = fence_vec[0];
    let _max = fence_vec[Opts::from_args().number];

    if let Err(e) = read_csv(file) {
        eprintln!("{}", e);
    }

    return vec![0.0, 99.0]; //placeholder return
}

fn read_csv(path: String) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut index_vec = vec![0.0];

    for result in reader.records() {
        let record = result?;
        let value: f64 = record[0].parse().unwrap();
        index_vec.push(value);
    }

    Ok(index_vec)
}

fn main() {
    let opt = Opts::from_args();
    // println!("{:#?}", opt);

    let fence_vec = find_fenceposts(opt.start, opt.end, opt.number);
    bucket_maxes(fence_vec, opt.file);
}

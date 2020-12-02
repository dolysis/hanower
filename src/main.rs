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
    number: u64,

    #[structopt(short, long)]
    file: String,
}

fn find_fenceposts(mut start: f64, mut end: f64, gaps: u64) -> Vec<f64> {
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
        println!("Gaps value too small.");
        return vec![start, end];
    }
}

// uses find_fenceposts() vec to identify buckets
// and then pulls the index number & row data from the max index
// in those buckets
fn bucket_maxes(fence_vec: Vec<f64>, file: String) -> Vec<f64> {
    match read_csv(file) {
        Err(e) => eprintln!("{}", e),
        Ok(index_vec) => {
            let _min = fence_vec[0];
            let _max = fence_vec[(Opts::from_args().number as usize) - 1];
            let _bucket_vec: Vec<f64> = vec![];

            return index_vec; //placeholder return
        }
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

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    // All functions (excluding main), structs and traits that are defined above
    // can be used in this module
    use super::*;

    /// Catch all error type, can easily use '?' and .into() with it
    type AnyError = Box<dyn std::error::Error + Send + 'static>;
    /// Typedef of the Results our #[test] functions return
    type TestResult = std::result::Result<(), AnyError>;

    /*
     * You have two tasks:
     *
     *  * Pass the tests
     *      1. Replace to the todo! in `fence_fn` with an actual function
     *      2. Iterate until your function passes all three tests
     *  * Change `FenceArgs` to only allow valid inputs
     *      1. Implement `FenceArgs::try_new`
     *      2. Move any error handling you can out of the fence function, as your inputs
     *         are guaranteed to be valid
     *
     * Every time you complete something, remove the TODO. When all TODO's
     * are gone, you've finished!
     *
     * -------------------------------------------------------------------------------------------
     *
     * For reference this is how I calculated the expected values in `test_data`:
     *
     *  -- INPUTS
     *      * $START  : The low value
     *      * $END    : The high value
     *      * $COUNT  : The number of fences to divide $START..=$END into
     *
     * -- OUTPUT
     *      * $OUTPUT : An array containing $COUNT fence points
     *
     * -- FUNCTIONS
     *      * ln      : See [f64::ln](https://doc.rust-lang.org/std/primitive.f64.html#method.ln)
     *      * exp     : See [f64::exp](https://doc.rust-lang.org/std/primitive.f64.html#method.ln)
     *
     * -- ALGORITHM
     * $CEILING := ln($END - $START + 1)
     * $NLOG    := $CEILING / $COUNT
     *
     * for $IDX in 1..=$COUNT:
     *      $EXP := exp($NLOG * $IDX)
     *      $POST    := $EXP + $START - 1
     *      $OUTPUT  += $POST
     *
     * return $OUTPUT
     */

    /// Small struct for carrying around fence function arguments
    #[derive(Debug, Clone, Copy)]
    struct FenceArgs {
        pub start: f64,
        pub end: f64,
        pub count: u64,
    }

    impl FenceArgs {
        fn new(start: f64, end: f64, count: u64) -> Self {
            Self { start, end, count }
        }

        /// TODO: How could we only allow valid inputs to create a new Self?
        #[allow(dead_code, unused_variables)]
        fn try_new(start: f64, end: f64, count: u64) -> Result<Self, AnyError> {
            unimplemented!("Work in progress!!")
        }
    }

    /// Wrapper function for passing around our actual fence post function
    ///
    /// TODO: replace the insides of the function with one that passes all of the tests below
    fn fence_fn(args: FenceArgs) -> Result<Vec<f64>, AnyError> {
        #[allow(unused_variables)]
        let your_fn = |start, end, count| {
            todo!(
                "Replace me with a function that finds the fences, \
             using the given arguments and returning a type compatible \
             with this function's returned value"
            )
        };

        your_fn(args.start, args.end, args.count)
    }

    /* --- TESTS --- */

    #[test]
    /// Checks that the fence function correctly detects and refuses invalid input values.
    ///
    /// TODO: Hmm, could we change the arguments passed into it to guarantee the inputs
    /// are valid somehow?
    fn start_after_end_err() -> TestResult {
        let args = FenceArgs::new(10.0, 1.0, 5);

        let test: Result<Vec<f64>, AnyError> = fence_fn(args);

        // Assert that bad inputs lead to an error
        assert!(test.is_err());

        test.map(|_| ())
    }

    #[test]
    /// Checks that the fence function correctly detects and refuses invalid count values
    ///
    /// TODO: Hmm, could we change the arguments passed into it to guarantee the inputs
    /// are valid somehow?
    fn count_less_than_two_err() -> TestResult {
        let args = FenceArgs::new(1.0, 10.0, 1);

        let test: Result<Vec<f64>, AnyError> = fence_fn(args);

        // Assert that a bad count leads to an error
        assert!(test.is_err());

        test.map(|_| ())
    }

    #[test]
    /// Runs the fence function against a series of precomputed data sets checking that
    /// all of the actual outputs match the expected values
    fn hanoi_algorithm_static_data() -> TestResult {
        let test_values = test_data().into_iter();

        // For each set of args and precomputed outputs
        for (args, expected_list) in test_values {
            // Generate the actual outputs
            let actual_list = fence_fn(args)?;

            // For each expected and actual data sets
            actual_list.iter().zip(expected_list.iter()).enumerate().try_for_each(|(idx, (&actual, &expected))| {
                // Round the actual item
                let rounded = actual.round() as i64;

                // Check that the rounded item matches the precomputed item
                if rounded != expected {
                    let msg = format!(
                        "@{} => Expected {}, received {} ({})\nExpected Values | {:?}\nActual Values  | {:?}",
                        idx, expected, rounded, actual, expected_list, actual_list
                    );

                    error(msg)?;
                }

                Ok(())
            })?;

            // Assert the lists are the same length, if they aren't the actual data is wrong
            assert_output_length(actual_list.len(), expected_list.len())?
        }

        Ok(())
    }

    /* --- HELPER FUNCTIONS -- */

    fn assert_output_length(actual: usize, expected: usize) -> TestResult {
        if let false = actual == expected {
            let msg = format!("Expected {} fences, but received: {}", expected, actual);

            return error(msg);
        }

        Ok(())
    }

    fn error(msg: impl ToString) -> TestResult {
        let err: Box<dyn std::error::Error + Send + Sync + 'static> = Box::from(msg.to_string());
        Err(err)
    }

    fn test_data() -> Vec<(FenceArgs, Vec<i64>)> {
        vec![
            (FenceArgs::new(1.0, 16.0, 4), vec![2, 4, 8, 16]),
            (
                FenceArgs::new(100.0, 1000.0, 15),
                vec![
                    101, 102, 103, 105, 109, 114, 123, 137, 158, 192, 246, 330, 462, 672, 1000,
                ],
            ),
            (
                FenceArgs::new(3.0, 72.0, 9),
                vec![4, 5, 6, 9, 13, 19, 29, 46, 72],
            ),
            (FenceArgs::new(-19.0, 12.0, 3), vec![-17, -10, 12]),
            (
                FenceArgs::new(-11000.0, -1200.0, 16),
                vec![
                    -10999, -10998, -10995, -10991, -10983, -10970, -10945, -10902, -10825, -10689,
                    -10446, -10016, -9252, -7894, -5483, -1200,
                ],
            ),
        ]
    }
}

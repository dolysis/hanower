use super::*;

use bybucket::ByBucket;

mod buckets;
mod bybucket;

#[derive(Debug, StructOpt)]
pub struct Range {
    #[structopt(long, default_value = "0")]
    start: i64,

    #[structopt(long, default_value = "256")]
    end: i64,

    #[structopt(long, default_value = "2")]
    count: u64,

    #[structopt(short = "M", long = "maxByBucket", conflicts_with = "min-by-bucket")]
    max_by_bucket: bool,

    #[structopt(short = "m", long = "minByBucket")]
    min_by_bucket: bool,

    range: Vec<i64>,
}

impl Runner for Range {
    type Config = Options;

    fn run(
        &mut self,
        dst: &mut dyn std::io::Write,
        _config: Option<Self::Config>,
    ) -> Result<(), color_eyre::eyre::Report> {
        let interval = core::Interval::new(self.start as f64, self.end as f64, self.count)?;

        let output = if self.max_by_bucket {
            ByBucket::new(interval).select_max(self.range.iter().copied())
        } else if self.min_by_bucket {
            ByBucket::new(interval).select_min(self.range.iter().copied())
        } else {
            let low = interval.low().round() as i64;
            let high = interval.high().round() as i64;
            self.range
                .iter()
                .filter(|&&item| low <= item && high >= item)
                .copied()
                .collect()
        };

        for &number in &output {
            write!(dst, "{} ", number)?;
        }
        writeln!(dst)?;

        Ok(())
    }
}

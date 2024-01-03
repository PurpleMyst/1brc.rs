use indicatif::{ProgressIterator, ProgressStyle};
use rand::prelude::*;
use std::{
    fs::File,
    io::{prelude::*, BufWriter},
};

use color_eyre::{eyre::format_err, Result};

const ROWS: usize = 1_000_000_000;

const TEMPLATE: &str =
    "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} ETA {eta_precise} ({per_sec})";

#[derive(Debug)]
struct WeatherStation {
    id: &'static str,
    distr: rand_distr::Normal<f64>,
}

impl WeatherStation {
    fn new(id: &'static str, mean: f64) -> Self {
        Self {
            id,
            distr: rand_distr::Normal::new(mean, 10.0).unwrap(),
        }
    }

    fn measurement(&self) -> f64 {
        self.distr.sample(&mut rand::thread_rng())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let re = regex::Regex::new(r#"new WeatherStation\("([^*]+)", ([^)]+)\)"#).unwrap();
    let stations = include_str!("stations.txt")
        .lines()
        .map(|line| {
            re.captures(line)
                .map(|cap| {
                    WeatherStation::new(
                        cap.get(1).unwrap().as_str(),
                        cap.get(2).unwrap().as_str().parse().unwrap(),
                    )
                })
                .ok_or_else(|| format_err!("Invalid line: {line:?}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut writer = BufWriter::new(File::create("measurements.txt")?);
    for _ in (0..ROWS).progress_with_style(ProgressStyle::default_bar().template(TEMPLATE)?) {
        let station = stations.choose(&mut rand::thread_rng()).unwrap();
        writeln!(writer, "{};{}", station.id, station.measurement())?;
    }

    Ok(())
}

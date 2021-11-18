mod trainer;
mod utils;

use std::{cmp::Ordering, path::Path};

use structopt::StructOpt;
use vinyana::{self, NeuralNetwork};

use crate::utils::min_max;

#[derive(StructOpt)]
struct Ops {
    #[structopt()]
    date: String,
    #[structopt()]
    impute: f32,
}

fn main() {
    let cli = Ops::from_args();

    let input = cli.date.split("-").collect::<Vec<_>>();
    let impute = cli.impute;

    let nm = NeuralNetwork::load(Path::new("month.bin")).unwrap();
    let nd = NeuralNetwork::load(Path::new("day.bin")).unwrap();

    let year = input[0].parse::<f32>().unwrap();
    let month = input[1];

    let month_as_f32 = if month == "?" {
        let result = nm.predict(vec![min_max(impute as f32, 1.0, 3.0)]);
        result
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap() as f32
            + 1.0
    } else {
        month.parse::<f32>().unwrap()
    };

    let result = nd.predict(vec![
        min_max(year, 1954.0, 2200.0),
        min_max(month_as_f32, 1.0, 12.0),
        min_max(impute as f32, 1.0, 3.0),
    ]);

    println!("{}", result);

    let day = result
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap()
        + 1;

    println!("{}-{}-{}", year as u32, month_as_f32 as u32, day);
}

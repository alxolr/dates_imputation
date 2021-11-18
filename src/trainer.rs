use core::panic;
use std::{path::Path, time::SystemTime};

use rand::{prelude::SliceRandom, thread_rng};
use vinyana::NeuralNetwork;

use crate::utils::{is_leap_year, min_max};

pub fn train_month() {
    let mut rng = thread_rng();
    let mut month_trainer_scenarios: Vec<(Vec<f32>, Vec<f32>)> = vec![
        (
            vec![1.0f32],
            vec![
                1.0f32, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
        ),
        (
            vec![2.0f32],
            vec![
                0.0f32, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
        ),
        (
            vec![3.0f32],
            vec![
                0.0f32, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        ),
    ]
    .into_iter()
    .map(|(input, output)| {
        let input = input.iter().map(|x| min_max(*x, 1.0, 3.0)).collect();

        (input, output)
    })
    .collect();

    let mut month_mind = NeuralNetwork::new(1, 3, 12, 0.05);

    let epochs = 100000;

    for _ in 0..epochs {
        month_trainer_scenarios.shuffle(&mut rng);
        let (inputs, targets) = month_trainer_scenarios.first().unwrap();
        month_mind.train(inputs.clone(), targets.clone())
    }

    month_mind.save(Path::new("month.bin")).unwrap();
}

pub fn train_day() {
    let mut rng = thread_rng();
    // let mut day_mind = NeuralNetwork::load(Path::new("day.bin")).unwrap();
    let mut day_mind = NeuralNetwork::new(3, 64, 31, 0.05);

    let mut day_trainer_scenarios: Vec<(Vec<f32>, Vec<f32>)> = gen_dataset()
        .into_iter()
        .map(|(input, output)| {
            let year = min_max(input[0] as f32, 1954.0, 2200.0);
            let month = min_max(input[1] as f32, 1.0, 12.0);
            let imputation = min_max(input[2] as f32, 1.0, 3.0);

            (vec![year, month, imputation], output)
        })
        .collect();

    // let mut day_mind = NeuralNetwork::new(3, 31, 31, 0.05);
    let epochs = 10000;
    let mut snapshot = 0;
    for e in 0..epochs {
        snapshot += 1;
        println!("epoch {}", e);
        let start = SystemTime::now();
        day_trainer_scenarios.shuffle(&mut rng);

        day_trainer_scenarios.iter().for_each(|(inputs, targets)| {
            day_mind.train(inputs.clone(), targets.clone());
        });

        let trained_time = SystemTime::now();
        println!("{}", trained_time.duration_since(start).unwrap().as_secs());

        if snapshot == 100 {
            println!("save step");
            day_mind.save(Path::new("day.bin")).unwrap();
            snapshot = 0;
        }
    }

    day_mind.save(Path::new("day.bin")).unwrap();
}

pub fn day_at(index: usize) -> Vec<f32> {
    let mut vec: Vec<f32> = Vec::with_capacity(31);

    for i in 0..31 {
        if i == index {
            vec.push(1.0);
        } else {
            vec.push(0.0)
        }
    }

    vec
}

pub fn gen_dataset() -> Vec<(Vec<u32>, Vec<f32>)> {
    // 30 day months
    // April 4
    // June 6
    // September 9
    // November 11

    // 28 day months
    // February 2

    // 29 day months leap year
    // February 2

    // Everything else is 31

    // 2004, 2, .3 -> 29
    let mut vec = vec![];
    // let impute = 3;
    let range = 1954..=2100u32;
    // let month = 2;
    for year in range {
        for month in 1..=12 {
            for impute in 1..=3 {
                let day = match impute {
                    1 => 1,
                    2 => 15,
                    3 => {
                        let day = if is_leap_year(year) && month == 2 {
                            29
                        } else if month == 2 {
                            28
                        } else if vec![4, 6, 9, 11].contains(&month) {
                            30
                        } else {
                            31
                        };

                        day
                    }
                    _ => panic!("Do not care"),
                };

                vec.push((vec![year, month, impute], day_at(day - 1)))
            }
        }
    }

    vec
}

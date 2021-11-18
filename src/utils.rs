pub fn min_max(x: f32, min: f32, max: f32) -> f32 {
    (x - min) / (max - min)
}

pub fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && year % 100 != 100 || year % 400 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leap_years() {
        let scenarios = vec![(2024u32, true), (2020, true), (2019, false), (1994, false)];

        scenarios.iter().for_each(|(input, target)| {
            assert_eq!(is_leap_year(*input), *target);
        })
    }
}

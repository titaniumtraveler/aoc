use advent_of_code::day::Day;
use advent_of_code::{calendar, day::Config};
use thiserror::Error;

type NomError<T> = nom::error::Error<T>;

pub fn year_2022(config: Config, str: &str) -> Result<u32, Year2022Error> {
    calendar!(match (config, str) {})
}

#[derive(Debug, Error)]
pub enum Year2022Error {}

use crate::from_enum;
use crate::year_2022::day02::Day02;
use advent_of_code::day::Day;
use advent_of_code::{calendar, day::Config};
use thiserror::Error;

mod day02;

type NomError<T> = nom::error::Error<T>;

pub fn year_2022(config: Config, str: &str) -> Result<u32, Year2022Error> {
    calendar!(match (config, str) {
        2 => Day02,
    })
}

#[derive(Debug, Error)]
pub enum Year2022Error<'a> {
    #[error(transparent)]
    Day02(day02::Error<'a>),
}

from_enum!(Year2022Error::Day02(day02::Error<'a>));

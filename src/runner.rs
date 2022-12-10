use crate::{
    from_enum,
    year_2022::{year_2022, Year2022Error},
};
use advent_of_code::day::{Config, Runner};
use thiserror::Error;

#[derive(Debug)]
pub struct AocRunner;

impl<'a> Runner<'a> for AocRunner {
    type Error = AocRunnerError;

    fn run(&mut self, config: Config, str: &'a str) -> Result<u32, Self::Error> {
        match config.year {
            2022 => Ok(year_2022(config, str)?),
            _ => unreachable!("Year {} isn't implemented", config.year),
        }
    }
}

#[derive(Debug, Error)]
pub enum AocRunnerError {
    #[error(transparent)]
    Year2022(Year2022Error<'a>),
}

from_enum!(AocRunnerError::Year2022(Year2022Error<'a>));

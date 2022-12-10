use advent_of_code::day::{Config, Runner};
use thiserror::Error;

#[derive(Debug)]
pub struct AocRunner;

impl<'a> Runner<'a> for AocRunner {
    type Error = AocRunnerError;

    fn run(&mut self, config: Config, str: &'a str) -> Result<u32, Self::Error> {
        match config.year {
            _ => unreachable!("Year {} isn't implemented", config.year),
        }
    }
}

#[derive(Debug, Error)]
pub enum AocRunnerError {}

use crate::{from_enum, year_2022::NomError};
use advent_of_code::day::Day;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{eof, map_opt},
    multi::many1,
    sequence::tuple,
    Finish, IResult,
};
use thiserror::Error;

pub struct Day02;

impl<'a> Day<'a> for Day02 {
    type Parser = Parser;
    type Error = Error<'a>;

    fn parse(input: &'a str) -> Result<Self::Parser, Self::Error> {
        let (_input, vec) = many1(parse_line)(input).finish()?;
        Ok(Parser(vec))
    }

    fn part_1(parser: Self::Parser) -> Result<u32, Self::Error> {
        Ok(parser.0.iter().map(|(left, right)| right.fight(left)).sum())
    }

    fn part_2(parser: Self::Parser) -> Result<u32, Self::Error> {
        Ok(parser
            .0
            .iter()
            .map(|(left, right)| left.win_lose_draw(right))
            .sum())
    }
}

#[derive(Debug, Error)]
pub enum Error<'a> {
    #[error("failed to parse input for day02")]
    ParserError(NomError<&'a str>),
}

from_enum!(Error::ParserError(NomError<&'a str>));

#[derive(Debug)]
pub struct Parser(Vec<(RockPaperScissors, RockPaperScissors)>);

fn parse_line(input: &str) -> IResult<&str, (RockPaperScissors, RockPaperScissors)> {
    let (input, (left, _, right, _)) = tuple((
        map_opt(one_of("ABC"), RockPaperScissors::parse),
        tag(" "),
        map_opt(one_of("XYZ"), RockPaperScissors::parse),
        alt((tag("\n"), eof)),
    ))(input)?;

    Ok((input, (left, right)))
}

#[derive(Debug)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    fn fight(&self, other: &Self) -> u32 {
        use RockPaperScissors::*;
        // Game win/lose/draw
        (match (self, other) {
            // lose
            (Scissors, Rock) | (Rock, Paper) | (Paper, Scissors) => 0,
            // draw
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
            // win
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 6,
        } + match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        })
    }

    fn win_lose_draw(&self, other: &Self) -> u32 {
        use RockPaperScissors::*;
        // rock means lose
        // paper means draw
        // scissors means win
        match (self, other) {
            (Rock, Paper) | (Paper, Rock) | (Scissors, Scissors) => Rock,
            (Rock, Scissors) | (Paper, Paper) | (Scissors, Rock) => Paper,
            (Rock, Rock) | (Paper, Scissors) | (Scissors, Paper) => Scissors,
        }
        .fight(self)
    }

    fn parse(input: char) -> Option<Self> {
        use RockPaperScissors::*;
        match input {
            'A' | 'X' => Some(Rock),
            'B' | 'Y' => Some(Paper),
            'C' | 'Z' => Some(Scissors),
            _ => None,
        }
    }
}

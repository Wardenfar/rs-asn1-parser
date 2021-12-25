use std::num::ParseIntError;

use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::combinator::{map_res, opt, value};
use nom::sequence::pair;
use nom_supreme::tag::complete::tag;

use crate::char::is_digit;
use crate::{In, Res};

pub(crate) fn number(input: In) -> Res<u64> {
    map_res(take_while(is_digit), |str: &str| {
        str.parse().map_err(|e| nom::Err::Failure(e))
    })(input)
}

pub(crate) fn signed_number(input: In) -> Res<i64> {
    map_res(
        pair(
            opt(alt((value(-1, tag("-")), value(1, tag("+"))))),
            take_while(is_digit),
        ),
        |(sign, str): (Option<i64>, &str)| -> Result<i64, nom::Err<ParseIntError>> {
            let mut v: i64 = str.parse().map_err(|e| nom::Err::Failure(e))?;
            if let Some(sign) = sign {
                v *= sign;
            }
            Ok(v)
        },
    )(input)
}

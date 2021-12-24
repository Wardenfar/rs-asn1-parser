use nom::bytes::complete::take_while;
use nom::combinator::{map_res};

use crate::char::is_digit;
use crate::{In, Res};

pub(crate) fn number(input: In) -> Res<u64> {
    map_res(take_while(is_digit), |str: &str| {
        str.parse().map_err(|e| nom::Err::Failure(e))
    })(input)
}

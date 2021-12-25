use std::borrow::Borrow;

use nom::bytes::complete::take_while;
use nom::character::complete::{line_ending, not_line_ending};
use nom::combinator::{map, opt};
use nom::sequence::{delimited, pair, tuple};
use nom::AsChar;
use nom_supreme::tag::complete::tag;

use crate::{In, Res};

pub fn is_upper(c: char) -> bool {
    c.is_ascii_uppercase()
}

pub fn is_lower(c: char) -> bool {
    c.is_ascii_lowercase()
}

pub fn is_alpha_lower<C: Borrow<char>>(c: C) -> bool {
    c.borrow().is_alpha() && c.borrow().is_ascii_lowercase()
}

pub fn is_digit<C: Borrow<char>>(c: C) -> bool {
    c.borrow().is_digit(10)
}

pub fn is_alpha_upper<C: Borrow<char>>(c: C) -> bool {
    c.borrow().is_alpha() && c.borrow().is_ascii_uppercase()
}

pub fn is_space<C: Borrow<char>>(c: C) -> bool {
    match c.borrow() {
        ' ' | '\t' | '\r' | '\n' => true,
        _ => false,
    }
}

pub fn is_line_ending<C: Borrow<char>>(c: C) -> bool {
    match c.borrow() {
        ' ' | '\t' | '\r' | '\n' => true,
        _ => false,
    }
}

pub fn space(input: In) -> Res<()> {
    map(
        pair(
            take_while(is_space),
            opt(tuple((tag("--"), not_line_ending, line_ending, space))),
        ),
        |_| (),
    )(input)
}

pub fn space_tag<'a>(t: &'static str) -> impl FnMut(In<'a>) -> Res<'a, ()> {
    map(delimited(space, tag(t), space), |_| ())
}

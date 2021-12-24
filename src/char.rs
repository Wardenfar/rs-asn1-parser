use std::borrow::Borrow;

use nom::bytes::complete::take_while;
use nom::combinator::map;

use nom::AsChar;

use crate::{In, Res};

pub(crate) fn is_upper(c: char) -> bool {
    c.is_ascii_uppercase()
}

pub(crate) fn is_lower(c: char) -> bool {
    c.is_ascii_lowercase()
}

pub(crate) fn is_alpha_lower<C: Borrow<char>>(c: C) -> bool {
    c.borrow().is_alpha() && c.borrow().is_ascii_lowercase()
}

pub(crate) fn is_alpha_upper<C: Borrow<char>>(c: C) -> bool {
    c.borrow().is_alpha() && c.borrow().is_ascii_uppercase()
}

pub(crate) fn is_space<C: Borrow<char>>(c: C) -> bool {
    match c.borrow() {
        ' ' | '\t' => true,
        _ => false,
    }
}

pub(crate) fn space(input: In) -> Res<()> {
    map(take_while(is_space), |_| ())(input)
}
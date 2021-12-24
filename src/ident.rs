use nom::bytes::complete::take_while;
use nom::character::complete::anychar;
use nom::combinator::{map, verify};
use nom::sequence::pair;
use nom::AsChar;

use crate::char::{is_alpha_lower, is_alpha_upper};
use crate::{In, Res};

#[derive(Debug, Clone)]
pub struct Ident {
    name: String,
}

impl Ident {
    pub(crate) fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }

    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}

pub(crate) fn top_ident(input: In) -> Res<Ident> {
    map(
        pair(
            verify(anychar, |c: &char| is_alpha_upper(c)),
            take_while(is_ident),
        ),
        |(first_letter, rest)| Ident::new(format!("{}{}", first_letter, rest)),
    )(input)
}

pub(crate) fn ident(input: In) -> Res<Ident> {
    map(
        pair(
            verify(anychar, |c: &char| is_alpha_lower(c)),
            take_while(is_ident),
        ),
        |(first_letter, rest)| Ident::new(format!("{}{}", first_letter, rest)),
    )(input)
}

fn is_ident(c: char) -> bool {
    c.is_alphanum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{test_full_failed, test_full_ok};

    #[test]
    fn top_ident_test() {
        test_full_ok("Abc", top_ident);
        test_full_ok("Abc012", top_ident);
        test_full_ok("A42abcP", top_ident);
        test_full_ok("Abc", top_ident);
        test_full_failed("abc", top_ident);
        test_full_failed("0bc", top_ident);
        test_full_failed("Abc_", top_ident);
    }

    #[test]
    fn ident_test() {
        test_full_ok("abc", ident);
        test_full_ok("abc012", ident);
        test_full_ok("a42abcP", ident);
        test_full_ok("abc", ident);
        test_full_failed("Abc", ident);
        test_full_failed("0bc", ident);
        test_full_failed("abc_", ident);
    }
}

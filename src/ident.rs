use nom::bytes::complete::take_while;
use nom::character::complete::anychar;
use nom::combinator::{map, verify};
use nom::sequence::pair;
use nom::AsChar;
use serde::{Deserialize, Serialize};

use crate::char::{is_alpha_lower, is_alpha_upper};
use crate::validation::Validation;
use crate::{In, Res, ValidRes};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ident {
    name: String,
}

impl Validation for Ident {
    fn check(&self) -> ValidRes<()> {
        Ok(())
    }
}

impl Ident {
    pub fn new<S: Into<String>>(name: S) -> Self {
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
    use nom::combinator::all_consuming;

    use super::*;

    #[test]
    fn top_ident_test() {
        all_consuming(top_ident)("Abc").unwrap();
        all_consuming(top_ident)("Abc012").unwrap();
        all_consuming(top_ident)("A42abcP").unwrap();
        all_consuming(top_ident)("abc").unwrap_err();
        all_consuming(top_ident)("0bc").unwrap_err();
        all_consuming(top_ident)("Abc_").unwrap_err();
    }

    #[test]
    fn ident_test() {
        all_consuming(ident)("abc").unwrap();
        all_consuming(ident)("abc012").unwrap();
        all_consuming(ident)("a42abcP").unwrap();
        all_consuming(ident)("Abc").unwrap_err();
        all_consuming(ident)("0bc").unwrap_err();
        all_consuming(ident)("abc_").unwrap_err();
    }
}

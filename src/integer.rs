use nom::combinator::{map, opt};
use nom::sequence::preceded;
use serde::{Deserialize, Serialize};

use crate::char::space;
use crate::enumerated::{enumerated, Enumerated};
use crate::{In, Res, ValidRes, Validation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Integer {
    pub enumerated: Option<Enumerated>,
}

pub(crate) fn integer(input: In) -> Res<Integer> {
    map(opt(preceded(space, enumerated)), |e| Integer {
        enumerated: e,
    })(input)
}

impl Validation for Integer {
    fn check(&self) -> ValidRes<()> {
        self.enumerated.check()
    }
}

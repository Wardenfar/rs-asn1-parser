use nom::combinator::map;
use nom::sequence::{preceded, terminated, tuple};
use nom_supreme::tag::complete::tag;
use serde::{Deserialize, Serialize};

use crate::char::{space, space_tag};
use crate::enumerated::{enumerated, Enumerated};
use crate::ident::{top_ident, Ident};
use crate::{In, Res};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopField {
    name: Ident,
    kind: FieldKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldKind {
    Enum(Enumerated),
}

pub fn top_field(input: In) -> Res<TopField> {
    map(
        tuple((
            terminated(top_ident, space_tag("::=")),
            preceded(terminated(tag("ENUMERATED"), space), enumerated),
        )),
        |(name, kind)| TopField { name, kind },
    )(input)
}

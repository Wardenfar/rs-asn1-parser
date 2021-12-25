use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded};
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;

use crate::char::{space, space_tag};
use crate::field::FieldKind;
use crate::ident::{ident, Ident};
use crate::number::signed_number;
use crate::{In, Res};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enumerated {
    items: Vec<EnumItem>,
    extensible: bool,
    extensions: Vec<EnumItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnumItem {
    Name(Ident),
    NamedNumber(Ident, i64),
}

pub fn enumerated(input: In) -> Res<FieldKind> {
    map(
        delimited(
            space_tag("{"),
            pair(
                separated_list1(space_tag(","), enum_item),
                opt(preceded(
                    pair(space_tag(","), space_tag("...")),
                    opt(preceded(
                        space_tag(","),
                        separated_list1(space_tag(","), enum_item),
                    )),
                )
                .context("enum extension")),
            ),
            preceded(space, tag("}")),
        )
        .context("enum"),
        |(items, extensions)| {
            let extensible = extensions.is_some();
            let extensions = extensions.unwrap_or(Some(vec![])).unwrap_or(vec![]);
            FieldKind::Enum(Enumerated {
                items,
                extensible,
                extensions,
            })
        },
    )(input)
}

pub fn enum_item(input: In) -> Res<EnumItem> {
    let first = map(
        pair(
            ident,
            delimited(space_tag("("), signed_number, space_tag(")")),
        ),
        |(name, val)| EnumItem::NamedNumber(name, val),
    );

    let second = map(ident, |name| EnumItem::Name(name));

    context("enum item", alt((first, second)))(input)
}

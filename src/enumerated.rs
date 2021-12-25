use std::collections::HashMap;

use nom::branch::alt;
use nom::combinator::map;
use nom::error::context;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded};
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

use crate::char::{space, space_tag};
use crate::field::FieldKind;
use crate::ident::{ident, Ident};
use crate::number::signed_number;
use crate::validation::Validation;
use crate::{Asn1ParserError, In, Res, ValidRes};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enumerated {
    items: Vec<EnumItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnumItem {
    Name(Ident),
    NamedNumber(Ident, i64),
}

pub(crate) fn enumerated(input: In) -> Res<FieldKind> {
    map(
        delimited(
            space_tag("{"),
            separated_list1(space_tag(","), enum_item),
            preceded(space, tag("}")),
        )
        .context("enum"),
        |items| FieldKind::Enum(Enumerated { items }),
    )(input)
}

fn enum_item(input: In) -> Res<EnumItem> {
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

impl Validation for Enumerated {
    fn check(&self) -> ValidRes<()> {
        let mut result: HashMap<i64, &Ident> = HashMap::new();

        for (idx, item) in self.items.iter().enumerate() {
            let (val, name) = match item {
                EnumItem::Name(name) => (idx as i64, name),
                EnumItem::NamedNumber(name, val) => (*val, name),
            };
            if let Some(conflict) = result.get(&val) {
                return Err(Asn1ParserError::EnumConflictValue(
                    val,
                    name.as_str().into(),
                    conflict.as_str().into(),
                ));
            } else {
                result.insert(val, name);
            }
        }
        Ok(())
    }
}

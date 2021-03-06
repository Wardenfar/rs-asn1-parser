use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

use crate::char::{space, space_tag};
use crate::field::{top_field, TopField};
use crate::ident::{ident, top_ident, Ident};
use crate::number::number;
use crate::validation::Validation;
use crate::{In, Res, ValidRes};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: Ident,
    pub header: Option<ModuleHeader>,
    pub body: ModuleBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleBody {
    pub top_fields: Vec<TopField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleHeader {
    pub keys: Vec<HeaderKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeaderKey {
    Named(Ident),
    Number(u64),
    NamedNumber(Ident, u64),
}

pub(crate) fn module(input: In) -> Res<Module> {
    let p = tuple((
        delimited(space, top_ident, space),
        opt(module_header),
        space_tag("DEFINITIONS"),
        space_tag("::="),
        space_tag("BEGIN"),
        terminated(module_body, space),
        space_tag("END"),
    ))
    .all_consuming();

    map(p, |(name, header, _, _, _, body, _)| Module {
        name,
        header,
        body,
    })(input)
}

fn module_header(input: In) -> Res<ModuleHeader> {
    map(
        delimited(
            space_tag("{"),
            separated_list0(space, header_key),
            space_tag("}"),
        ),
        |keys| ModuleHeader { keys },
    )(input)
}

fn header_key(input: In) -> Res<HeaderKey> {
    let named_number = map(
        pair(
            ident,
            delimited(space_tag("("), number, preceded(space, tag(")"))),
        ),
        |(ident, n)| HeaderKey::NamedNumber(ident, n),
    );
    let num = map(number, |n| HeaderKey::Number(n));
    let named = map(ident, |i| HeaderKey::Named(i));
    alt((named_number, named, num))(input)
}

fn module_body(input: In) -> Res<ModuleBody> {
    map(
        separated_list0(space, top_field).context("module body"),
        |top_fields| ModuleBody { top_fields },
    )(input)
}

impl Validation for Module {
    fn check(&self) -> ValidRes<()> {
        self.name.check()?;
        self.header.check()?;
        self.body.check()
    }
}

impl Validation for ModuleHeader {
    fn check(&self) -> ValidRes<()> {
        self.keys.check()
    }
}

impl Validation for ModuleBody {
    fn check(&self) -> ValidRes<()> {
        self.top_fields.check()
    }
}

impl Validation for HeaderKey {
    fn check(&self) -> ValidRes<()> {
        match self {
            HeaderKey::Named(name) => name.check(),
            HeaderKey::Number(_) => Ok(()),
            HeaderKey::NamedNumber(name, _) => name.check(),
        }
    }
}

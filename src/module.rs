use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt, success};
use nom::multi::{separated_list0};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};

use crate::char::{space, space_tag};
use crate::ident::{ident, top_ident, Ident};
use crate::number::number;
use crate::{In, Res};

#[derive(Debug, Clone)]
pub struct Module {
    name: Ident,
    header: Option<ModuleHeader>,
}

#[derive(Debug, Clone)]
pub struct ModuleBody {}

#[derive(Debug, Clone)]
pub struct ModuleHeader {
    keys: Vec<HeaderKey>,
}

#[derive(Debug, Clone)]
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
    ));

    map(p, |(name, header, _, _, _, _body, _)| Module {
        name,
        header,
    })(input)
}

pub(crate) fn module_header(input: In) -> Res<ModuleHeader> {
    map(
        delimited(
            space_tag("{"),
            separated_list0(space, header_key),
            space_tag("}"),
        ),
        |keys| ModuleHeader { keys },
    )(input)
}

pub(crate) fn header_key(input: In) -> Res<HeaderKey> {
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

pub(crate) fn module_body(input: In) -> Res<ModuleBody> {
    success(ModuleBody {})(input)
}

#[cfg(test)]
mod tests {
    use crate::test_util::test_full_ok;

    use super::*;

    #[test]
    fn test_module() {
        let m = test_full_ok(include_str!("../resources/00-empty-OK.asn1"), module);
        assert_eq!(m.name.as_str(), "ModuleTestEmpty");
        assert!(m.header.is_none());

        let m = test_full_ok(include_str!("../resources/01-empty-OK.asn1"), module);
        assert_eq!(m.name.as_str(), "ModuleTestEmpty");
        assert!(m.header.is_some());
        let header = m.header.unwrap();
        assert_eq!(header.keys.len(), 11)
    }
}

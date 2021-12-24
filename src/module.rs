use nom::bytes::complete::tag;
use nom::combinator::{map, success};
use nom::sequence::{terminated, tuple};

use crate::char::space;
use crate::ident::{top_ident, Ident};
use crate::{In, Res};

#[derive(Debug, Clone)]
pub struct Module {
    name: Ident,
}

#[derive(Debug, Clone)]
pub struct ModuleBody {}

pub(crate) fn module(input: In) -> Res<Module> {
    let p = tuple((
        terminated(
            terminated(top_ident, space),
            tuple((
                terminated(tag("DEFINITIONS"), space),
                terminated(tag("::="), space),
                terminated(tag("BEGIN"), space),
            )),
        ),
        terminated(module_body, terminated(tag("END"), space)),
    ));

    map(p, |(name, _body)| Module { name })(input)
}

pub(crate) fn module_body(input: In) -> Res<ModuleBody> {
    success(ModuleBody {})(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::test_full_ok;

    #[test]
    fn test_module() {
        let m = test_full_ok("ModuleTestEmpty DEFINITIONS ::= BEGIN END", module);
        assert_eq!(m.name.as_str(), "ModuleTestEmpty")
    }
}

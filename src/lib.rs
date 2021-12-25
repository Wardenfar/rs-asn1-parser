use nom_supreme::error::ErrorTree;

use crate::module::{module, Module};
use crate::validation::{Asn1ParserError, ValidRes, Validation};

pub(crate) mod char;
pub mod enumerated;
pub mod field;
pub mod ident;
pub mod module;
pub mod number;
pub mod validation;

type In<'a> = &'a str;
type Res<'a, O> = nom::IResult<In<'a>, O, ErrorTree<In<'a>>>;

pub fn parse_asn1_file<'a>(input: In) -> ValidRes<Module> {
    let module = module(input).map_err(|e| Asn1ParserError::NomError(format!("{:#?}", e)))?;
    let module = module.1;
    module.check()?;
    Ok(module)
}

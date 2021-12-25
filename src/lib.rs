use nom_supreme::error::ErrorTree;

pub mod char;
pub mod enumerated;
pub mod field;
pub mod ident;
pub mod module;
pub mod number;

type In<'a> = &'a str;
type Res<'a, O> = nom::IResult<In<'a>, O, ErrorTree<In<'a>>>;

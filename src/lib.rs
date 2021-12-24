pub(crate) mod char;
pub(crate) mod ident;
pub(crate) mod module;

#[cfg(test)]
pub(crate) mod test_util;

type In<'a> = &'a str;
type Res<'a, O> = nom::IResult<In<'a>, O>;

fn main() {
    println!("Hello, world!");
}

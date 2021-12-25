use thiserror::Error;

pub type ValidRes<T> = Result<T, Asn1ParserError>;

pub trait Validation {
    fn check(&self) -> ValidRes<()>;
}

impl<T> Validation for Vec<T>
where
    T: Validation,
{
    fn check(&self) -> ValidRes<()> {
        for i in self {
            i.check()?;
        }
        Ok(())
    }
}

impl<T> Validation for Option<T>
where
    T: Validation,
{
    fn check(&self) -> ValidRes<()> {
        if let Some(inner) = self {
            inner.check()?;
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum Asn1ParserError {
    #[error("Two items of the same enum conflict values")]
    EnumConflictValue(i64, String, String),
    #[error("Enum extensions must be ordered")]
    EnumExtensionOrder,
    #[error("Parsing Error")]
    NomError(String),
}

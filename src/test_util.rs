use crate::{In, Res};

pub(crate) fn test_full_ok<'a, F, O>(input: In<'a>, f: F) -> O
where
    F: Fn(In<'a>) -> Res<'a, O>,
{
    let res = f(input);
    match res {
        Ok(o) => {
            assert_eq!(o.0.len(), 0);
            return o.1;
        }
        Err(e) => {
            panic!("{}", e)
        }
    }
}

pub(crate) fn test_full_failed<'a, F, O>(input: In<'a>, f: F)
where
    F: Fn(In<'a>) -> Res<'a, O>,
{
    match f(input) {
        Ok(o) => assert!(!o.0.is_empty()),
        Err(_) => {}
    }
}

use std::convert::Infallible;

use crate::data::traits::{Attachable, Combinable, Pure, Response, ResultConvertable};

use super::sure::Sure;

impl Response for () {}

impl Pure for () {
    type Value = ();
    fn pure(value: Self::Value) -> Self {}
    fn unwrap(self) -> Self::Value {}
}

impl<Res> Combinable<Res> for ()
where
    Res: Response,
{
    type Output = Res;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Res,
    {
        response()
    }
}

impl<Val> Attachable<Val> for () {
    type Output = Sure<Val>;

    fn attach_to_response(self, value: Val) -> Self::Output {
        Sure(value)
    }
}

impl ResultConvertable for () {
    type Value = ();
    type Error = Infallible;
    type WithVal<Val> = ();

    fn ok(value: Self::Value) -> Self {
        let _ = value;
    }

    fn err(error: Self::Error) -> Self {
        let _ = error;
    }

    fn into_result(self) -> Result<Self::Value, Self::Error> {
        Ok(())
    }
}

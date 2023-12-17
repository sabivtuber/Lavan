use std::io::repeat;

use crate::stream::traits::Stream;

use super::prelude::Sure;

pub trait Response {}

pub trait Data: Response {
    type Value;
    type WithVal<Val>: Data;

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val;

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>;
}

pub trait Pure: Response {
    type Value;

    fn pure(value: Self::Value) -> Self;
    fn unwrap(self) -> Self::Value;
}

pub trait Exceptional: Response {
    type Error;
    type WithErr<Err>: Exceptional;

    fn map_err<Fun, Err>(self, f: Fun) -> Self::WithErr<Err>
    where
        Fun: FnOnce(Self::Error) -> Err;
}

pub trait Combinable<Res>: Response
where
    Res: Response,
{
    type Output: Response;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Res;
}

pub trait Disjoinable<Res>: Response
where
    Res: Response,
{
    type Output: Response;

    fn disjoin_response<Fun, Rec, Str>(
        self,
        response: Fun,
        recover: Rec,
        stream: &mut Str,
    ) -> Self::Output
    where
        Fun: FnOnce(&mut Str) -> Res,
        Rec: FnOnce(&mut Str),
        Str: Stream;
}

pub trait Recoverable: Response {
    fn recover_response<Rec, Str>(self, on_residual: Rec, stream: &mut Str) -> Self
    where
        Rec: FnOnce(&mut Str),
        Str: Stream;
}

pub trait Ignorable: Response {
    type Output: Response;
    fn ignore_response(self) -> Self::Output;
}

pub trait Attachable<Val>: Response {
    type Output: Data;
    fn attach_to_response(self, value: Val) -> Self::Output;
}

pub trait Optionable: Recoverable {
    type Output: Response;
    fn opt_response(self) -> Self::Output;
}

pub trait ResultConvertable: Response {
    type Value;
    type Error;
    type WithVal<Val>: ResultConvertable;

    fn ok(collector: Self::Value) -> Self;
    fn err(error: Self::Error) -> Self;
    fn into_result(self) -> Result<Self::Value, Self::Error>;
}

pub trait UnerringConvertable: ResultConvertable {
    type Infallible: Pure<Value = Self::Value>;
}

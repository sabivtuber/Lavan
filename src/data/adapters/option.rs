use crate::{
    data::traits::{Combinable, Data, Disjoinable, Ignorable, Optionable, Recoverable, Response},
    stream::traits::Stream,
};

use super::pure::Pure;

impl<T> Response for Option<T> {}

impl<T> Data for Option<T> {
    type Value = T;
    type WithVal<Val> = Option<Val>;

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val,
    {
        self.map(f)
    }

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>,
    {
        self.and_then(f)
    }
}

impl<Val> Combinable<()> for Option<Val> {
    type Output = Self;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce(),
    {
        let value = self?;
        response();
        Some(value)
    }
}

impl<Val0, Val1> Combinable<Option<Val1>> for Option<Val0> {
    type Output = Option<(Val0, Val1)>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Option<Val1>,
    {
        self.and_then(|val0| response().map(|val1| (val0, val1)))
    }
}

impl<Val0, Val1> Combinable<Pure<Val1>> for Option<Val0> {
    type Output = Option<(Val0, Val1)>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Pure<Val1>,
    {
        Some((self?, response().value()))
    }
}

impl<Val> Disjoinable<Option<Val>> for Option<Val> {
    type Output = Option<Val>;

    fn disjoin_response<Fun, Rec, Str>(
        self,
        response: Fun,
        recover: Rec,
        stream: &mut Str,
    ) -> Self::Output
    where
        Fun: FnOnce(&mut Str) -> Option<Val>,
        Rec: FnOnce(&mut Str),
        Str: Stream,
    {
        match self {
            Some(value) => Some(value),
            None => {
                recover(stream);
                response(stream)
            }
        }
    }
}

impl<Val> Recoverable for Option<Val> {
    fn recover_residual<Rec, Str>(self, on_residual: Rec, stream: &mut Str) -> Self
    where
        Rec: FnOnce(&mut Str),
        Str: Stream,
    {
        match self {
            Some(value) => Some(value),
            None => {
                on_residual(stream);
                None
            }
        }
    }
}

impl<Val> Ignorable for Option<Val> {
    type Output = Option<()>;

    fn ignore_response(self) -> Self::Output {
        self.map(|_| ())
    }
}

impl<Val> Optionable for Option<Val> {
    type Output = Pure<Option<Val>>;

    fn opt_response(self) -> Self::Output {
        Pure(self)
    }
}

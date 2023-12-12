use std::fmt::Debug;

use crate::{
    data::traits::{
        Attachable, Combinable, Disjoinable, Exceptional, Optionable, Recoverable, Response,
    },
    stream::traits::Stream,
};

use super::pure::Pure;

pub struct Effect<E>(pub Result<(), E>);

impl<E> Effect<E> {
    pub fn ok() -> Self {
        Self(Ok(()))
    }

    pub fn err(error: E) -> Self {
        Self(Err(error))
    }

    pub fn into_result(self) -> Result<(), E> {
        self.0
    }

    pub fn unwrap(self)
    where
        E: Debug,
    {
        self.into_result().unwrap();
    }
}

impl<E> From<Result<(), E>> for Effect<E> {
    fn from(value: Result<(), E>) -> Self {
        Self(value)
    }
}

impl<E> From<Effect<E>> for Result<(), E> {
    fn from(value: Effect<E>) -> Self {
        value.0
    }
}

impl<T> Response for Effect<T> {}

impl<T> Exceptional for Effect<T> {
    type Error = T;
    type WithErr<Err> = Effect<Err>;

    fn map_err<Fun, Err>(self, f: Fun) -> Self::WithErr<Err>
    where
        Fun: FnOnce(Self::Error) -> Err,
    {
        self.into_result().map_err(f).into()
    }
}

impl<Err> Combinable<()> for Effect<Err> {
    type Output = Self;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce(),
    {
        match self.into_result() {
            Ok(()) => {
                response();
                Effect::ok()
            }
            Err(error) => Effect::err(error),
        }
    }
}

impl<Err, Val> Combinable<Pure<Val>> for Effect<Err> {
    type Output = Result<Val, Err>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Pure<Val>,
    {
        self.into_result()?;
        Ok(response().value())
    }
}

impl<Err> Combinable<Effect<Err>> for Effect<Err> {
    type Output = Self;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Effect<Err>,
    {
        match self.into_result() {
            Ok(()) => response(),
            Err(error) => Self::err(error),
        }
    }
}

impl<Err, Val> Combinable<Result<Val, Err>> for Effect<Err> {
    type Output = Result<Val, Err>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Result<Val, Err>,
    {
        self.into_result()?;
        response()
    }
}

impl<Err> Disjoinable<Effect<Err>> for Effect<Err> {
    type Output = Effect<(Err, Err)>;

    fn disjoin_response<Fun, Rec, Str>(
        self,
        response: Fun,
        recover: Rec,
        stream: &mut Str,
    ) -> Self::Output
    where
        Fun: FnOnce(&mut Str) -> Effect<Err>,
        Rec: FnOnce(&mut Str),
        Str: Stream,
    {
        match self.into_result() {
            Ok(()) => Effect::ok(),
            Err(error0) => {
                recover(stream);
                response(stream).map_err(|error1| (error0, error1))
            }
        }
    }
}

impl<Err> Recoverable for Effect<Err> {
    fn recover_residual<Rec, Str>(self, on_residual: Rec, stream: &mut Str) -> Self
    where
        Rec: FnOnce(&mut Str),
        Str: Stream,
    {
        match self.into_result() {
            Ok(()) => Effect::ok(),
            Err(error) => {
                on_residual(stream);
                Effect::err(error)
            }
        }
    }
}

impl<Val, Err> Attachable<Val> for Effect<Err> {
    type Output = Result<Val, Err>;

    fn attach_to_response(self, value: Val) -> Self::Output {
        self.into_result().map(|()| value)
    }
}

impl<Val> Optionable for Effect<Val> {
    type Output = ();

    fn opt_response(self) -> Self::Output {
        let _ = self;
    }
}

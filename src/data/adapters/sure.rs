use std::convert::Infallible;

use crate::data::traits::{Combinable, Data, Ignorable, Pure, Response, ResultConvertable};

use super::effect::Effect;

pub struct Sure<T>(pub T);

impl<T> Sure<T> {
    pub fn value(self) -> T {
        self.0
    }

    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Response for Sure<T> {}

impl<T> Data for Sure<T> {
    type Value = T;
    type WithVal<Val> = Sure<Val>;

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val,
    {
        Sure(f(self.value()))
    }

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>,
    {
        f(self.value())
    }
}

impl<T> Pure for Sure<T> {
    type Value = T;

    fn pure(value: Self::Value) -> Self {
        Sure(value)
    }

    fn unwrap(self) -> Self::Value {
        self.value()
    }
}

impl<Val> Combinable<()> for Sure<Val> {
    type Output = Self;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce(),
    {
        response();
        self
    }
}

impl<Val0, Val1> Combinable<Sure<Val1>> for Sure<Val0> {
    type Output = Sure<(Val0, Val1)>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Sure<Val1>,
    {
        Sure((self.value(), response().value()))
    }
}

impl<Val0, Val1> Combinable<Option<Val1>> for Sure<Val0> {
    type Output = Option<(Val0, Val1)>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Option<Val1>,
    {
        Some((self.value(), response()?))
    }
}

impl<Val, Err> Combinable<Effect<Err>> for Sure<Val> {
    type Output = Result<Val, Err>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Effect<Err>,
    {
        response().into_result()?;
        Ok(self.value())
    }
}

impl<Val0, Val1, Err> Combinable<Result<Val1, Err>> for Sure<Val0> {
    type Output = Result<(Val0, Val1), Err>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Result<Val1, Err>,
    {
        let value = response()?;
        Ok((self.value(), value))
    }
}

impl<Val> Ignorable for Sure<Val> {
    type Output = ();

    fn ignore_response(self) -> Self::Output {
        let _ = self;
    }
}

impl<Val> ResultConvertable for Sure<Val> {
    type Value = Val;
    type Error = Infallible;
    type WithVal<Col> = Sure<Col>;

    fn ok(value: Self::Value) -> Self {
        Sure(value)
    }

    fn err(error: Self::Error) -> Self {
        unreachable!()
    }

    fn into_result(self) -> Result<Self::Value, Self::Error> {
        Ok(self.value())
    }
}

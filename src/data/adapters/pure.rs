use crate::data::traits::{Combinable, Data, Ignorable, Response};

use super::effect::Effect;

pub struct Pure<T>(pub T);

impl<T> Pure<T> {
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

impl<T> Response for Pure<T> {}

impl<T> Data for Pure<T> {
    type Value = T;
    type WithVal<Val> = Pure<Val>;

    fn map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Val,
    {
        Pure(f(self.value()))
    }

    fn flat_map<Fun, Val>(self, f: Fun) -> Self::WithVal<Val>
    where
        Fun: FnOnce(Self::Value) -> Self::WithVal<Val>,
    {
        f(self.value())
    }
}

impl<Val> Combinable<()> for Pure<Val> {
    type Output = Self;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce(),
    {
        response();
        self
    }
}

impl<Val0, Val1> Combinable<Pure<Val1>> for Pure<Val0> {
    type Output = Pure<(Val0, Val1)>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Pure<Val1>,
    {
        Pure((self.value(), response().value()))
    }
}

impl<Val0, Val1> Combinable<Option<Val1>> for Pure<Val0> {
    type Output = Option<(Val0, Val1)>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Option<Val1>,
    {
        Some((self.value(), response()?))
    }
}

impl<Val, Err> Combinable<Effect<Err>> for Pure<Val> {
    type Output = Result<Val, Err>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Effect<Err>,
    {
        response().into_result()?;
        Ok(self.value())
    }
}

impl<Val0, Val1, Err> Combinable<Result<Val1, Err>> for Pure<Val0> {
    type Output = Result<(Val0, Val1), Err>;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Result<Val1, Err>,
    {
        let value = response()?;
        Ok((self.value(), value))
    }
}

impl<Val> Ignorable for Pure<Val> {
    type Output = ();

    fn ignore_response(self) -> Self::Output {
        let _ = self;
    }
}

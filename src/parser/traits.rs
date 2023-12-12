use super::adapters::and::And;
use super::adapters::attach::Attach;
use super::adapters::flat_map::FlatMap;
use super::adapters::ignore::Ignore;
use super::adapters::map::Map;
use super::adapters::map_err::MapErr;
use super::adapters::non_terminal::NonTerminal;
use super::adapters::opt::Opt;
use super::adapters::or::Or;
use super::util::assoc::{err, val};
use crate::data::prelude::*;
use crate::stream::traits::Stream;

pub trait ParserOnce<Str>
where
    Str: Stream,
{
    type Output: Response;

    fn parse_stream_once(self, input: &mut Str) -> Self::Output;

    // Unary Adapters

    fn map<Fun, Out>(self, f: Fun) -> Map<Self, Fun>
    where
        Self: Sized,
        Self::Output: Data,
        Fun: FnOnce(val![Self]) -> Out,
    {
        Map::new(self, f)
    }

    fn map_err<Fun, Err>(self, f: Fun) -> MapErr<Self, Fun>
    where
        Self: Sized,
        Self::Output: Exceptional,
        Fun: Fn(err![Self]) -> Err,
    {
        MapErr::new(self, f)
    }

    fn flat_map<Fun, Val>(self, f: Fun) -> FlatMap<Self, Fun, Val>
    where
        Self: Sized,
        Self::Output: Data + Exceptional,
        Fun: Fn(val![Self]) -> val![Self<Val>],
    {
        FlatMap::new(self, f)
    }

    fn ignore(self) -> Ignore<Self>
    where
        Self: Sized,
        Self::Output: Ignorable,
    {
        Ignore::new(self)
    }

    fn attach<Val>(self, value: Val) -> Attach<Self, Val>
    where
        Self: Sized,
        Self::Output: Attachable<Val>,
    {
        Attach::new(self, value)
    }

    fn non_terminal(self) -> NonTerminal<Self>
    where
        Self: Sized,
        Self::Output: Recoverable,
    {
        NonTerminal::new(self)
    }

    fn opt(self) -> Opt<Self>
    where
        Self: Sized,
        Self::Output: Optionable,
    {
        Opt::new(self)
    }

    // Binary Adapters

    fn and<Par>(self, parser: Par) -> And<Self, Par>
    where
        Self: Sized,
        Par: Parser<Str>,
        Self::Output: Combinable<Par::Output>,
    {
        And::new(self, parser)
    }

    fn or<Par>(self, parser: Par) -> Or<Self, Par>
    where
        Self: Sized,
        Par: Parser<Str>,
        Self::Output: Disjoinable<Par::Output>,
    {
        Or::new(self, parser)
    }
}

pub trait ParserMut<Str>: ParserOnce<Str>
where
    Str: Stream,
{
    fn parse_stream_mut(&mut self, input: &mut Str) -> Self::Output;
}

pub trait Parser<Str>: ParserMut<Str>
where
    Str: Stream,
{
    fn parse_stream(&self, input: &mut Str) -> Self::Output;
}

impl<Str, Out, Fun> Parser<Str> for Fun
where
    Str: Stream,
    Out: Response,
    Fun: Fn(&mut Str) -> Out,
{
    fn parse_stream(&self, input: &mut Str) -> Self::Output {
        self(input)
    }
}

impl<Str, Out, Fun> ParserMut<Str> for Fun
where
    Str: Stream,
    Out: Response,
    Fun: FnMut(&mut Str) -> Out,
{
    fn parse_stream_mut(&mut self, input: &mut Str) -> Self::Output {
        self(input)
    }
}

impl<Str, Out, Fun> ParserOnce<Str> for Fun
where
    Str: Stream,
    Out: Response,
    Fun: FnOnce(&mut Str) -> Out,
{
    type Output = Out;

    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        self(input)
    }
}

pub trait Parse {
    type Input: Stream;

    fn parse(input: &mut Self::Input) -> Self;
}

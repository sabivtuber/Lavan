use super::adapters::{
    and::And,
    attach::Attach,
    ignore::Ignore,
    map::Map,
    map_err::MapErr,
    non_terminal::NonTerminal,
    opt::Opt,
    or::Or,
    repeat::{mode::*, *},
    try_map::TryMap,
};
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

    fn try_map<Fun, Val>(self, f: Fun) -> TryMap<Self, Fun, Val>
    where
        Self: Sized,
        Self::Output: Data + Exceptional,
        Fun: Fn(val![Self]) -> val![Self<Val>],
    {
        TryMap::new(self, f)
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

    // repeat

    fn repeat(self) -> Repeat<Self>
    where
        Self: Sized,
        Self: ParserMut<Str>,
        Self::Output: UnerringConvertable,
    {
        Repeat::new(self, UntilErr)
    }

    fn repeat_eoi(self) -> RepeatEOI<Self>
    where
        Self: Sized,
        Self: ParserMut<Str>,
        Self::Output: ResultConvertable,
    {
        RepeatEOI::new(self, UntilEOI)
    }

    fn repeat_min(self, count: usize) -> RepeatMin<Self>
    where
        Self: Sized,
        Self: ParserMut<Str>,
        Self::Output: ResultConvertable,
    {
        RepeatMin::new(self, Minimum(count))
    }

    fn repeat_min_eoi(self, count: usize) -> RepeatMinEOI<Self>
    where
        Self: Sized,
        Self: ParserMut<Str>,
        Self::Output: ResultConvertable,
    {
        RepeatMinEOI::new(self, MinimumEOI(count))
    }

    fn repeat_max(self, count: usize) -> RepeatMax<Self>
    where
        Self: Sized,
        Self: ParserMut<Str>,
        Self::Output: UnerringConvertable,
    {
        RepeatMax::new(self, Maximum(count))
    }

    fn repeat_exact(self, count: usize) -> RepeatExact<Self>
    where
        Self: Sized,
        Self: ParserMut<Str>,
        Self::Output: ResultConvertable,
    {
        RepeatExact::new(self, Exact(count))
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

impl<Str, Out> Parser<Str> for fn(&mut Str) -> Out
where
    Str: Stream,
    Out: Response,
{
    fn parse_stream(&self, input: &mut Str) -> Self::Output {
        self(input)
    }
}

impl<Str, Out> ParserMut<Str> for fn(&mut Str) -> Out
where
    Str: Stream,
    Out: Response,
{
    fn parse_stream_mut(&mut self, input: &mut Str) -> Self::Output {
        self(input)
    }
}

impl<Str, Out> ParserOnce<Str> for fn(&mut Str) -> Out
where
    Str: Stream,
    Out: Response,
{
    type Output = Out;

    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        self(input)
    }
}

impl<Str, Par> Parser<Str> for &Par
where
    Str: Stream,
    Par: Parser<Str>,
{
    fn parse_stream(&self, input: &mut Str) -> Self::Output {
        (*self).parse_stream(input)
    }
}

impl<Str, Par> ParserMut<Str> for &Par
where
    Str: Stream,
    Par: Parser<Str>,
{
    fn parse_stream_mut(&mut self, input: &mut Str) -> Self::Output {
        self.parse_stream(input)
    }
}

impl<Str, Par> ParserOnce<Str> for &Par
where
    Str: Stream,
    Par: Parser<Str>,
{
    type Output = Par::Output;

    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        self.parse_stream(input)
    }
}

impl<Str, Par> ParserMut<Str> for &mut Par
where
    Str: Stream,
    Par: ParserMut<Str>,
{
    fn parse_stream_mut(&mut self, input: &mut Str) -> Self::Output {
        (*self).parse_stream_mut(input)
    }
}

impl<Str, Par> ParserOnce<Str> for &mut Par
where
    Str: Stream,
    Par: ParserMut<Str>,
{
    type Output = Par::Output;

    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        self.parse_stream_mut(input)
    }
}

pub trait Parse {
    type Input: Stream;

    fn parse(input: &mut Self::Input) -> Self;
}

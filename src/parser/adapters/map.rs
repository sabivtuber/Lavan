use crate::data::prelude::*;
use crate::parser::prelude::*;
use crate::stream::traits::Stream;

pub type FnMap<Par, Val0, Val1> = Map<Par, fn(Val0) -> Val1>;

pub struct Map<Par, Fun> {
    parser: Par,
    function: Fun,
}

impl<Par, Fun> Map<Par, Fun> {
    pub(crate) fn new<Str, Val>(parser: Par, function: Fun) -> Map<Par, Fun>
    where
        Str: Stream,
        Par: ParserOnce<Str>,
        Par::Output: Data,
        Fun: FnOnce(val![Par]) -> Val,
    {
        Map { parser, function }
    }
}

impl<Str, Par, Fun, Val> Parser<Str> for Map<Par, Fun>
where
    Str: Stream,
    Par: Parser<Str>,
    Par::Output: Data,
    Fun: Fn(val![Par]) -> Val,
{
    fn parse_stream(&self, input: &mut Str) -> Self::Output {
        self.parser.parse_stream(input).map(&self.function)
    }
}

impl<Str, Par, Fun, Val> ParserMut<Str> for Map<Par, Fun>
where
    Str: Stream,
    Par: ParserMut<Str>,
    Par::Output: Data,
    Fun: FnMut(val![Par]) -> Val,
{
    fn parse_stream_mut(&mut self, input: &mut Str) -> Self::Output {
        self.parser.parse_stream_mut(input).map(&mut self.function)
    }
}

impl<Str, Par, Fun, Val> ParserOnce<Str> for Map<Par, Fun>
where
    Str: Stream,
    Par: ParserOnce<Str>,
    Par::Output: Data,
    Fun: FnOnce(val![Par]) -> Val,
{
    type Output = val![Par<Val>];

    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        self.parser.parse_stream_once(input).map(self.function)
    }
}

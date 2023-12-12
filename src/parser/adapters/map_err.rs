use crate::data::prelude::*;
use crate::parser::prelude::*;
use crate::stream::traits::Stream;

pub struct MapErr<Par, Fun> {
    parser: Par,
    function: Fun,
}

impl<Par, Fun> MapErr<Par, Fun> {
    pub(crate) fn new<Str, Err>(parser: Par, function: Fun) -> Self
    where
        Str: Stream,
        Par: ParserOnce<Str>,
        Par::Output: Exceptional,
        Fun: Fn(err![Par]) -> Err,
    {
        Self { parser, function }
    }
}

impl<Str, Par, Fun, Err> ParserOnce<Str> for MapErr<Par, Fun>
where
    Str: Stream,
    Par: ParserOnce<Str>,
    Par::Output: Exceptional,
    Fun: Fn(err![Par]) -> Err,
{
    type Output = err![Par<Err>];

    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        self.parser.parse_stream_once(input).map_err(&self.function)
    }
}

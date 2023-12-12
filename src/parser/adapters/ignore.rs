use crate::data::prelude::*;
use crate::parser::prelude::*;
use crate::stream::traits::Stream;

pub struct Ignore<Par> {
    parser: Par,
}

impl<Par> Ignore<Par> {
    pub(crate) fn new<Str>(parser: Par) -> Self
    where
        Str: Stream,
        Par: ParserOnce<Str>,
        Par::Output: Ignorable,
    {
        Self { parser }
    }
}

impl<Str, Par> ParserOnce<Str> for Ignore<Par>
where
    Str: Stream,
    Par: ParserOnce<Str>,
    Par::Output: Ignorable,
{
    type Output = <Par::Output as Ignorable>::Output;

    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        self.parser.parse_stream_once(input).ignore_response()
    }
}

use crate::data::prelude::*;
use crate::parser::prelude::*;
use crate::stream::traits::Stream;

pub struct Opt<Par> {
    parser: Par,
}

impl<Par> Opt<Par> {
    pub(crate) fn new<Str>(parser: Par) -> Self
    where
        Str: Stream,
        Par: ParserOnce<Str>,
        Par::Output: Optionable,
    {
        Self { parser }
    }
}

impl<Str, Par> ParserOnce<Str> for Opt<Par>
where
    Str: Stream,
    Par: ParserOnce<Str>,
    Par::Output: Optionable,
{
    type Output = <Par::Output as Optionable>::Output;

    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        self.parser
            .non_terminal()
            .parse_stream_once(input)
            .opt_response()
    }
}

use crate::data::prelude::*;
use crate::parser::prelude::*;
use crate::stream::traits::Stream;

pub struct NonTerminal<Par> {
    parser: Par,
}

impl<Par> NonTerminal<Par> {
    pub(crate) fn new<Str>(parser: Par) -> Self
    where
        Str: Stream,
        Par: ParserOnce<Str>,
        Par::Output: Recoverable,
    {
        Self { parser }
    }
}

impl<Str, Par> ParserOnce<Str> for NonTerminal<Par>
where
    Str: Stream,
    Par: ParserOnce<Str>,
    Par::Output: Recoverable,
{
    type Output = Par::Output;

    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        let offset = input.offset();
        self.parser.parse_stream_once(input).recover_response(
            |input| {
                *input.offset_mut() = offset;
            },
            input,
        )
    }
}

impl<Str, Par> ParserMut<Str> for NonTerminal<Par>
where
    Str: Stream,
    Par: ParserMut<Str>,
    Par::Output: Recoverable,
{
    fn parse_stream_mut(&mut self, input: &mut Str) -> Self::Output {
        let offset = input.offset();
        self.parser.parse_stream_mut(input).recover_response(
            |input| {
                *input.offset_mut() = offset;
            },
            input,
        )
    }
}

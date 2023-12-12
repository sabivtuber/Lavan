use crate::data::prelude::*;
use crate::parser::prelude::*;
use crate::stream::traits::Stream;

pub struct Or<Par0, Par1> {
    parser0: Par0,
    parser1: Par1,
}

impl<Par0, Par1> Or<Par0, Par1> {
    pub(crate) fn new<Str>(parser0: Par0, parser1: Par1) -> Or<Par0, Par1>
    where
        Str: Stream,
        Par0: ParserOnce<Str>,
        Par1: ParserOnce<Str>,
        Par0::Output: Disjoinable<Par1::Output>,
    {
        Or { parser0, parser1 }
    }
}

#[cfg(feature = "either")]
use either::Either;

#[cfg(feature = "either")]
impl<Par0, Par1> Or<Par0, Par1> {
    pub fn either<Str>(self) -> Or<impl ParserOnce<Str>, impl ParserOnce<Str>>
    where
        Str: Stream,
        Par0: ParserOnce<Str>,
        Par0::Output: Data,
        Par1: ParserOnce<Str>,
        Par1::Output: Data,
    {
        let parser0 = self
            .parser0
            .map(Either::<<Par0::Output as Data>::Value, <Par1::Output as Data>::Value>::Left);
        let parser1 = self
            .parser1
            .map(Either::<<Par0::Output as Data>::Value, <Par1::Output as Data>::Value>::Right);
        Or { parser0, parser1 }
    }
}

impl<Str, Par0, Par1> ParserOnce<Str> for Or<Par0, Par1>
where
    Str: Stream,
    Par0: ParserOnce<Str>,
    Par1: ParserOnce<Str>,
    Par0::Output: Disjoinable<Par1::Output>,
{
    type Output = <Par0::Output as Disjoinable<Par1::Output>>::Output;

    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        let offset = input.offset();
        self.parser0.parse_stream_once(input).disjoin_response(
            |str| self.parser1.parse_stream_once(str),
            |str| *str.offset_mut() = offset,
            input,
        )
    }
}

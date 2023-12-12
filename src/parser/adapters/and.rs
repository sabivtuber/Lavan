use crate::data::prelude::*;
use crate::parser::prelude::*;
use crate::parser::util::assoc::err;
use crate::stream::traits::Stream;

pub struct And<Par0, Par1> {
    parser0: Par0,
    parser1: Par1,
}

impl<Par0, Par1> And<Par0, Par1> {
    pub(crate) fn new<Str>(parser0: Par0, parser1: Par1) -> And<Par0, Par1>
    where
        Str: Stream,
        Par0: ParserOnce<Str>,
        Par1: ParserOnce<Str>,
        Par0::Output: Combinable<Par1::Output>,
    {
        And { parser0, parser1 }
    }
}

impl<Par0, Par1> And<Par0, Par1> {
    #[cfg(feature = "either")]
    pub fn either_err<Str>(self) -> And<impl ParserOnce<Str>, impl ParserOnce<Str>>
    where
        Str: Stream,
        Par0: ParserOnce<Str>,
        Par0::Output: Exceptional,
        Par1: ParserOnce<Str>,
        Par1::Output: Exceptional,
    {
        use either::Either;

        And {
            parser0: self.parser0.map_err(Either::<err![Par0], err![Par1]>::Left),
            parser1: self
                .parser1
                .map_err(Either::<err![Par0], err![Par1]>::Right),
        }
    }
}

impl<Str, Par0, Par1> ParserOnce<Str> for And<Par0, Par1>
where
    Str: Stream,
    Par0: ParserOnce<Str>,
    Par1: ParserOnce<Str>,
    Par0::Output: Combinable<Par1::Output>,
{
    type Output = <Par0::Output as Combinable<Par1::Output>>::Output;

    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        self.parser0
            .parse_stream_once(input)
            .combine_response(|| self.parser1.parse_stream_once(input))
    }
}

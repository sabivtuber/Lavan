use crate::data::prelude::*;
use crate::parser::prelude::*;
use crate::stream::traits::Stream;

pub struct Attach<Par, Val> {
    parser: Par,
    value: Val,
}

impl<Par, Val> Attach<Par, Val> {
    pub(crate) fn new<Str>(parser: Par, value: Val) -> Self
    where
        Str: Stream,
        Par: ParserOnce<Str>,
        Par::Output: Attachable<Val>,
    {
        Self { parser, value }
    }
}

impl<Str, Par, Val> ParserOnce<Str> for Attach<Par, Val>
where
    Str: Stream,
    Par: ParserOnce<Str>,
    Par::Output: Attachable<Val>,
    Val: Clone,
{
    type Output = <Par::Output as Attachable<Val>>::Output;

    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        self.parser
            .parse_stream_once(input)
            .attach_to_response(self.value.clone())
    }
}

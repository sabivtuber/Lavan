use crate::data::prelude::*;
use crate::parser::prelude::*;
use crate::stream::traits::Stream;
use std::marker::PhantomData;

pub struct FlatMap<Par, Fun, Val> {
    parser: Par,
    function: Fun,
    _marker: PhantomData<Val>,
}

impl<Par, Fun, Val> FlatMap<Par, Fun, Val> {
    pub(crate) fn new<Str>(parser: Par, function: Fun) -> FlatMap<Par, Fun, Val>
    where
        Str: Stream,
        Par: ParserOnce<Str>,
        Par::Output: Data + Exceptional,
        Fun: Fn(val![Par]) -> val![Par<Val>],
    {
        FlatMap {
            parser,
            function,
            _marker: PhantomData,
        }
    }
}

impl<Str, Par, Fun, Val> ParserOnce<Str> for FlatMap<Par, Fun, Val>
where
    Str: Stream,
    Par: ParserOnce<Str>,
    Par::Output: Data + Exceptional,
    Fun: Fn(val![Par]) -> val![Par<Val>],
{
    type Output = val![Par<Val>];

    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        self.parser
            .parse_stream_once(input)
            .flat_map(&self.function)
    }
}

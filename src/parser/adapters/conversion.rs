use crate::data::prelude::*;
use crate::parser::prelude::*;
use crate::stream::traits::Stream;

pub struct Unit<Par> {
    parser: Par,
}

impl<Par> Unit<Par> {
    pub(crate) fn new(parser: Par) -> Self
    where
        Par: ParserOnce,
        Par::Output: Data,
    {
        Self { parser }
    }
}

impl<Par> ParserOnce for Unit<Par>
where
    Par: ParserOnce,
    Par::Output: Data,
{
    type Input = Par::Input;
    type Output = <Par::Output as Data>::WithVal<()>;

    fn parse_stream_once(&self, input: &mut Self::Input) -> Self::Output {
        self.parser.parse_stream_once(input).map(|_| ())
    }
}

use crate::data::prelude::*;
use crate::parser::prelude::*;
use crate::stream::traits::Stream;
use std::marker::PhantomData;

#[derive(Debug, Default)]
pub enum OneOrMul<T, Col = Vec<T>>
where
    Col: Collector<T>,
{
    #[default]
    Empty,
    One(T),
    Multiple(Col),
}

pub trait Collector<T>: Default {
    fn push(&mut self, value: T);
}

impl<T> Collector<T> for () {
    fn push(&mut self, _: T) {}
}

impl<T> Collector<T> for Vec<T> {
    #[inline(always)]
    fn push(&mut self, value: T) {
        self.push(value)
    }
}

impl Collector<char> for String {
    #[inline(always)]
    fn push(&mut self, ch: char) {
        self.push(ch)
    }
}

impl<'a> Collector<&'a str> for String {
    #[inline(always)]
    fn push(&mut self, str: &'a str) {
        self.push_str(str)
    }
}

impl<T, Col> Collector<T> for OneOrMul<T, Col>
where
    Col: Collector<T>,
{
    #[inline(always)]
    fn push(&mut self, value: T) {
        let one_or_mul = std::mem::take(self);

        *self = match one_or_mul {
            OneOrMul::Empty => OneOrMul::One(value),
            OneOrMul::One(first) => {
                let mut col = Col::default();
                col.push(first);
                col.push(value);
                OneOrMul::Multiple(col)
            }
            OneOrMul::Multiple(mut col) => {
                col.push(value);
                OneOrMul::Multiple(col)
            }
        };
    }
}

pub mod mode {
    use std::marker::PhantomData;

    #[derive(Clone, Copy, Debug)]
    pub struct UntilErr;

    #[derive(Clone, Copy, Debug)]
    pub struct UntilEOI;

    #[derive(Clone, Copy, Debug)]
    pub struct Minimum(pub(crate) usize);

    #[derive(Clone, Copy, Debug)]
    pub struct MinimumEOI(pub(crate) usize);

    #[derive(Clone, Copy, Debug)]
    pub struct Maximum(pub(crate) usize);

    #[derive(Clone, Copy, Debug)]
    pub struct Exact(pub(crate) usize);

    #[derive(Clone, Copy, Debug)]
    pub struct Inter<Mod, Int>(pub(crate) Mod, pub(crate) Int);
}

use mode::*;

pub type Repeat<Par, Col = ()> = Repeater<Par, UntilErr, Col>;
pub type RepeatEOI<Par, Col = ()> = Repeater<Par, UntilEOI, Col>;
pub type RepeatMin<Par, Col = ()> = Repeater<Par, Minimum, Col>;
pub type RepeatMinEOI<Par, Col = ()> = Repeater<Par, MinimumEOI, Col>;
pub type RepeatMax<Par, Col = ()> = Repeater<Par, Maximum, Col>;
pub type RepeatExact<Par, Col = ()> = Repeater<Par, Exact, Col>;

#[derive(Clone, Copy, Debug)]
pub struct Repeater<Par, Mod, Col = ()> {
    parser: Par,
    mode: Mod,
    collector: Col,
}

impl<Par, Mod> Repeater<Par, Mod> {
    pub(crate) fn new<Str>(parser: Par, mode: Mod) -> Self
    where
        Str: Stream,
        Par: ParserMut<Str>,
        Par::Output: ResultConvertable,
    {
        Self {
            parser,
            mode,
            collector: (),
        }
    }
}

impl<Str, Par, Col, Out> ParserOnce<Str> for Repeat<Par, Col>
where
    Str: Stream,
    Par: ParserMut<Str, Output = Out>,
    Col: Extend<Out::Value>,
    Out: Recoverable + UnerringConvertable<Value = Col>,
{
    type Output = Out::Infallible;

    #[inline]
    fn parse_stream_once(mut self, input: &mut Str) -> Self::Output {
        loop {
            match (&mut self.parser)
                .non_terminal()
                .parse_stream_mut(input)
                .into_result()
            {
                Ok(val) => self.collector.extend([val]),
                Err(_) => return <Out::Infallible as Pure>::pure(self.collector),
            }
        }
    }
}

impl<Str, Par, Col, Out> ParserOnce<Str> for RepeatEOI<Par, Col>
where
    Str: Stream,
    Par: ParserMut<Str, Output = Out>,
    Col: Extend<Out::Value>,
    Out: ResultConvertable<Value = Col>,
{
    type Output = Out;

    #[inline]
    fn parse_stream_once(mut self, input: &mut Str) -> Self::Output {
        loop {
            match input.peek() {
                Some(_) => {}
                None => return Out::ok(self.collector),
            }

            self.collector
                .extend([result!(self.parser.parse_stream_mut(input))]);
        }
    }
}

impl<Str, Par, Col, Out> ParserOnce<Str> for RepeatMax<Par, Col>
where
    Str: Stream,
    Par: ParserMut<Str, Output = Out>,
    Col: Extend<Out::Value>,
    Out: Recoverable + UnerringConvertable<Value = Col>,
{
    type Output = Out::Infallible;

    #[inline]
    fn parse_stream_once(mut self, input: &mut Str) -> Self::Output {
        for _ in 0..self.mode.0 {
            match (&mut self.parser)
                .non_terminal()
                .parse_stream_mut(input)
                .into_result()
            {
                Ok(val) => self.collector.extend([val]),
                Err(_) => break,
            }
        }
        <Out::Infallible as Pure>::pure(self.collector)
    }
}

impl<Str, Par, Col, Out> ParserOnce<Str> for RepeatExact<Par, Col>
where
    Str: Stream,
    Par: ParserMut<Str, Output = Out>,
    Col: Extend<Out::Value>,
    Out: ResultConvertable<Value = Col>,
{
    type Output = Out;

    #[inline]
    fn parse_stream_once(mut self, input: &mut Str) -> Self::Output {
        for _ in 0..self.mode.0 {
            self.collector
                .extend([result!(self.parser.parse_stream_mut(input))])
        }
        Out::ok(self.collector)
    }
}

impl<Str, Par, Col, Out> ParserOnce<Str> for RepeatMin<Par, Col>
where
    Str: Stream,
    Par: ParserMut<Str, Output = Out>,
    Col: Extend<Out::Value>,
    Out: Recoverable + UnerringConvertable<Value = Col>,
{
    type Output = Out;

    #[inline]
    fn parse_stream_once(mut self, input: &mut Str) -> Self::Output {
        let mut collector = result!((&mut self.parser)
            .repeat_exact(self.mode.0)
            .collector(self.collector)
            .parse_stream_once(input));
        Out::ok(
            self.parser
                .repeat()
                .collector(collector)
                .parse_stream_once(input)
                .unwrap(),
        )
    }
}

impl<Str, Par, Col, Out> ParserOnce<Str> for RepeatMinEOI<Par, Col>
where
    Str: Stream,
    Par: ParserMut<Str, Output = Out>,
    Col: Extend<Out::Value>,
    Out: UnerringConvertable<Value = Col>,
{
    type Output = Out;

    #[inline]
    fn parse_stream_once(mut self, input: &mut Str) -> Self::Output {
        let mut collector = result!((&mut self.parser)
            .repeat_exact(self.mode.0)
            .collector(self.collector)
            .parse_stream_once(input));
        self.parser
            .repeat_eoi()
            .collector(collector)
            .parse_stream_once(input)
    }
}

// Interspersed

/*impl<Str, Par, Int, Col, Out> ParserOnce<Str> for Repeater<Par, Inter<UntilErr, Int>, Col>
where
    Str: Stream,
    Par: ParserMut<Str, Output = Out>,
    Int: ParserMut<Str>,
    Col: Extend<Out::Value>,
    Out: Recoverable + Combinable<Int::Output> + UnerringConvertable<Value = Col>,
{
    type Output = Out;

    #[inline]
    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        let Inter(UntilErr, ref mut int) = self.mode;

        match self
            .parser
            .non_terminal()
            .parse_stream_mut(input)
            .into_result()
        {
            Ok(value) => self.collector.extend([value]),
            Err(_) => return Out::ok(self.collector),
        }

        int.and(self.parser)
            .repeat()
            .collector(self.collector)
            .parse_stream_once(input)
    }
}

impl<Str, Par, Int, Col> ParserOnce<Str> for Repeater<Par, Inter<Maximum, Int>, Col>
where
    Str: Stream,
    Par: ParserOnce<Str>,
    Int: ParserOnce<Str>,
    Col: Collector<Par::Output>,
{
    type Output = Pure<Col>;

    #[inline]
    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        let mut collector = std::mem::take(&mut self.collector);
        let Inter(Maximum(maximum), ref mut int) = self.mode;

        if maximum == 0 {
            return Ok(collector);
        }

        match self.parser.non_terminal().parse_stream_once(input) {
            Ok(value) => collector.push(value),
            Err(_) => return Ok(collector),
        }

        int.then(self.parser)
            .repeat_max(maximum - 1)
            .collector(collector)
            .parse_stream_once(input)
    }
}

impl<Str, Par, Int, Col> ParserOnce<Str> for Repeater<Par, Inter<Exact, Int>, Col>
where
    Str: Stream,
    Par: ParserOnce<Str>,
    Int: ParserOnce<Str>,
    Par::Output: Exceptional,
    Int::Output: Exceptional<Error = <Par::Output as Exceptional>::Error>,
    Col: Collector<Par::Output>,
{
    type Output = Result<Col, err![Par]>;

    #[inline]
    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        let mut collector = std::mem::take(&mut self.collector);
        let Inter(Exact(exact), ref mut int) = self.mode;

        if exact == 0 {
            return Ok(collector);
        }

        collector.push(self.parser.parse_stream_once(input)?);

        int.then(self.parser)
            .repeat_exact(exact - 1)
            .collector(collector)
            .parse_stream_once(input)
    }
}

impl<Str, Par, Int, Col> ParserOnce<Str> for Repeater<Par, Inter<Minimum, Int>, Col>
where
    Str: Stream,
    Par: ParserOnce<Str>,
    Int: ParserOnce<Str>,
    Par::Output: Exceptional,
    Int::Output: Exceptional<Error = <Par::Output as Exceptional>::Error>,
    Col: Collector<Par::Output>,
{
    type Output = Result<Col, err![Par]>;

    #[inline]
    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        let collector = std::mem::take(&mut self.collector);
        let Inter(Minimum(minimum), ref mut int) = self.mode;

        let collector = self
            .parser
            .repeat_exact(minimum)
            .collector(collector)
            .interspersed(int)
            .parse_stream_once(input)?;

        Ok(int
            .then(self.parser)
            .repeat()
            .collector(collector)
            .parse_stream_once(input)
            .unwrap())
    }
}

impl<Str, Par, Int, Col> ParserOnce<Str> for Repeater<Par, Inter<MinimumEOI, Int>, Col>
where
    Str: Stream,
    Par: ParserOnce<Str>,
    Int: ParserOnce<Str>,
    Par::Output: Exceptional,
    Int::Output: Exceptional<Error = <Par::Output as Exceptional>::Error>,
    Col: Collector<Par::Output>,
{
    type Output = Result<Col, err![Par]>;

    #[inline]
    fn parse_stream_once(self, input: &mut Str) -> Self::Output {
        let collector = std::mem::take(&mut self.collector);
        let Inter(MinimumEOI(minimum), ref mut int) = self.mode;

        let collector = self
            .parser
            .repeat_exact(minimum)
            .collector(collector)
            .interspersed(int)
            .parse_stream_once(input)?;

        Ok(int
            .then(self.parser)
            .repeat_eoi()
            .collector(collector)
            .parse_stream_once(input)?)
    }
}*/

impl<Par, Mod, Col> Repeater<Par, Mod, Col> {
    /*#[inline(always)]
    pub fn interspersed<Str>(self, parser: Int) -> Repeater<Par, Inter<Mod, Int>, Col> {
        Repeater {
            parser: self.parser,
            mode: Inter(self.mode, parser),
            collector: self.collector,
        }
    }*/

    #[inline(always)]
    pub fn collector<T, Str>(self, collector: T) -> Repeater<Par, Mod, T>
    where
        Str: Stream,
        Par: ParserMut<Str>,
    {
        Repeater {
            parser: self.parser,
            mode: self.mode,
            collector,
        }
    }

    #[inline(always)]
    pub fn collect<T, Str>(self) -> Repeater<Par, Mod, T>
    where
        T: Default,
        Str: Stream,
        Par: ParserMut<Str>,
        Par::Output: Data,
    {
        Repeater {
            parser: self.parser,
            mode: self.mode,
            collector: T::default(),
        }
    }

    #[inline(always)]
    pub fn to_vec<Str>(self) -> Repeater<Par, Mod, Vec<Par::Output>>
    where
        Str: Stream,
        Par: ParserMut<Str>,
        Par::Output: Data,
    {
        Repeater {
            parser: self.parser,
            mode: self.mode,
            collector: vec![],
        }
    }
}

impl<Par, Col> Repeat<Par, Col> {
    #[inline(always)]
    pub fn until_eoi(self) -> RepeatEOI<Par, Col> {
        Repeater {
            parser: self.parser,
            mode: UntilEOI,
            collector: self.collector,
        }
    }
}

impl<Par, Col> RepeatMin<Par, Col> {
    #[inline(always)]
    pub fn until_eoi(self) -> RepeatMinEOI<Par, Col> {
        Repeater {
            parser: self.parser,
            mode: MinimumEOI(self.mode.0),
            collector: self.collector,
        }
    }
}

macro_rules! result {
    ($expr:expr) => {
        match $expr.into_result() {
            Ok(it) => it,
            Err(err) => return Out::err(err),
        }
    };
}
use result;

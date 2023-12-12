use crate::{parse_stream_oncer::traits::ParserOnce, stream::traits::Stream};

use super::*;

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
pub struct Repeater<Par, Mod = UntilErr, Col = ()> {
    pub(in super::super) parse_stream_oncer: Par,
    pub(in super::super) mode: Mod,
    pub(in super::super) collector: Col,
}

impl<Par, Inp, Col> ParserOnce for Repeater<Par, UntilErr, Col>
where
    Par: ParserOnce,
    Col: Collector<Par::Output>,
{
    type Input = Par::Input;
    type Output = Col;
    type Error = Infallible;

    #[inline]
    fn parse_stream_once(&mut self, input: &mut Self::Input) -> Self::Output {
        let mut collector = std::mem::take(&mut self.collector);
        while let Ok(value) = self
            .parse_stream_oncer
            .as_ref()
            .non_terminal()
            .parse_stream_once(input)
        {
            collector.push(value);
        }
        Ok(collector)
    }
}

impl<Par, Inp, Col> ParserOnce for Repeater<Par, UntilEOI, Col>
where
    Par: ParserOnce,
    Col: Collector<Par::Output>,
{
    type Input = Par::Input;
    type Output = Col;
    type Error = Par::Error;

    #[inline]
    fn parse_stream_once(&mut self, input: &mut Self::Input) -> Self::Output {
        let mut collector = std::mem::take(&mut self.collector);

        loop {
            match input.peek_next() {
                Some(_) => {}
                None => return Ok(collector),
            }

            collector.push(self.parse_stream_oncer.as_ref().parse_stream_once(input)?);
        }
    }
}

impl<Par, Inp, Col> ParserOnce for Repeater<Par, Maximum, Col>
where
    Par: ParserOnce,
    Col: Collector<Par::Output>,
{
    type Input = Par::Input;
    type Output = Col;
    type Error = Infallible;

    #[inline]
    fn parse_stream_once(&mut self, input: &mut Self::Input) -> Self::Output {
        let mut collector = std::mem::take(&mut self.collector);
        for _ in 0..self.mode.0 {
            match self
                .parse_stream_oncer
                .as_ref()
                .non_terminal()
                .parse_stream_once(input)
            {
                Ok(value) => collector.push(value),
                Err(_) => break,
            }
        }
        Ok(collector)
    }
}

impl<Par, Inp, Col> ParserOnce for Repeater<Par, Exact, Col>
where
    Par: ParserOnce,
    Col: Collector<Par::Output>,
{
    type Input = Par::Input;
    type Output = Col;
    type Error = Par::Error;

    #[inline]
    fn parse_stream_once(&mut self, input: &mut Self::Input) -> Self::Output {
        let mut collector = std::mem::take(&mut self.collector);
        for _ in 0..self.mode.0 {
            collector.push(self.parse_stream_oncer.parse_stream_once(input)?);
        }
        Ok(collector)
    }
}

impl<Par, Inp, Col> ParserOnce for Repeater<Par, Minimum, Col>
where
    Par: ParserOnce,
    Col: Collector<Par::Output>,
{
    type Input = Par::Input;
    type Output = Col;
    type Error = Par::Error;

    #[inline]
    fn parse_stream_once(&mut self, input: &mut Self::Input) -> Self::Output {
        let parse_stream_oncer = &mut self.parse_stream_oncer;

        let collector = parse_stream_oncer
            .as_ref()
            .repeat_exact(self.mode.0)
            .collector(std::mem::take(&mut self.collector))
            .parse_stream_once(input)?;

        let collector = parse_stream_oncer
            .as_ref()
            .repeat()
            .collector(collector)
            .parse_stream_once(input)
            .unwrap();

        Ok(collector)
    }
}

impl<Par, Inp, Col> ParserOnce for Repeater<Par, MinimumEOI, Col>
where
    Par: ParserOnce,
    Col: Collector<Par::Output>,
{
    type Input = Par::Input;
    type Output = Col;
    type Error = Par::Error;

    #[inline]
    fn parse_stream_once(&mut self, input: &mut Self::Input) -> Self::Output {
        let parse_stream_oncer = &mut self.parse_stream_oncer;

        let collector = parse_stream_oncer
            .as_ref()
            .repeat_exact(self.mode.0)
            .collector(std::mem::take(&mut self.collector))
            .parse_stream_once(input)?;

        let collector = parse_stream_oncer
            .as_ref()
            .repeat_eoi()
            .collector(collector)
            .parse_stream_once(input)?;

        Ok(collector)
    }
}

// Interspersed

impl<Par, Inp, Int, Col> ParserOnce for Repeater<Par, Inter<UntilErr, Int>, Col>
where
    Par: ParserOnce,
    Int: ParserOnce<Inp, Error = Par::Error>,
    Col: Collector<Par::Output>,
{
    type Input = Par::Input;
    type Output = Col;
    type Error = Infallible;

    #[inline]
    fn parse_stream_once(&mut self, input: &mut Self::Input) -> Self::Output {
        let mut collector = std::mem::take(&mut self.collector);
        let Inter(_, ref mut int) = self.mode;

        match self
            .parse_stream_oncer
            .as_ref()
            .non_terminal()
            .parse_stream_once(input)
        {
            Ok(value) => collector.push(value),
            Err(_) => return Ok(collector),
        }

        int.as_ref()
            .then(self.parse_stream_oncer.as_ref())
            .repeat()
            .collector(collector)
            .parse_stream_once(input)
    }
}

impl<Par, Inp, Int, Col> ParserOnce for Repeater<Par, Inter<Maximum, Int>, Col>
where
    Par: ParserOnce,
    Int: ParserOnce<Inp, Error = Par::Error>,
    Col: Collector<Par::Output>,
{
    type Input = Par::Input;
    type Output = Col;
    type Error = Infallible;

    #[inline]
    fn parse_stream_once(&mut self, input: &mut Self::Input) -> Self::Output {
        let mut collector = std::mem::take(&mut self.collector);
        let Inter(Maximum(maximum), ref mut int) = self.mode;

        if maximum == 0 {
            return Ok(collector);
        }

        match self
            .parse_stream_oncer
            .as_ref()
            .non_terminal()
            .parse_stream_once(input)
        {
            Ok(value) => collector.push(value),
            Err(_) => return Ok(collector),
        }

        int.as_ref()
            .then(self.parse_stream_oncer.as_ref())
            .repeat_max(maximum - 1)
            .collector(collector)
            .parse_stream_once(input)
    }
}

impl<Par, Inp, Int, Col> ParserOnce for Repeater<Par, Inter<Exact, Int>, Col>
where
    Par: ParserOnce,
    Int: ParserOnce<Inp, Error = Par::Error>,
    Col: Collector<Par::Output>,
{
    type Input = Par::Input;
    type Output = Col;
    type Error = Par::Error;

    #[inline]
    fn parse_stream_once(&mut self, input: &mut Self::Input) -> Self::Output {
        let mut collector = std::mem::take(&mut self.collector);
        let Inter(Exact(exact), ref mut int) = self.mode;

        if exact == 0 {
            return Ok(collector);
        }

        collector.push(self.parse_stream_oncer.parse_stream_once(input)?);

        int.as_ref()
            .then(self.parse_stream_oncer.as_ref())
            .repeat_exact(exact - 1)
            .collector(collector)
            .parse_stream_once(input)
    }
}

impl<Par, Inp, Int, Col> ParserOnce for Repeater<Par, Inter<Minimum, Int>, Col>
where
    Par: ParserOnce,
    Int: ParserOnce<Inp, Error = Par::Error>,
    Col: Collector<Par::Output>,
{
    type Input = Par::Input;
    type Output = Col;
    type Error = Par::Error;

    #[inline]
    fn parse_stream_once(&mut self, input: &mut Self::Input) -> Self::Output {
        let collector = std::mem::take(&mut self.collector);
        let Inter(Minimum(minimum), ref mut int) = self.mode;

        let collector = self
            .parse_stream_oncer
            .as_ref()
            .repeat_exact(minimum)
            .collector(collector)
            .interspersed(int.as_ref())
            .parse_stream_once(input)?;

        Ok(int
            .as_ref()
            .then(self.parse_stream_oncer.as_ref())
            .repeat()
            .collector(collector)
            .parse_stream_once(input)
            .unwrap())
    }
}

impl<Par, Inp, Int, Col> ParserOnce for Repeater<Par, Inter<MinimumEOI, Int>, Col>
where
    Par: ParserOnce,
    Int: ParserOnce<Inp, Error = Par::Error>,
    Col: Collector<Par::Output>,
{
    type Input = Par::Input;
    type Output = Col;
    type Error = Par::Error;

    #[inline]
    fn parse_stream_once(&mut self, input: &mut Self::Input) -> Self::Output {
        let collector = std::mem::take(&mut self.collector);
        let Inter(MinimumEOI(minimum), ref mut int) = self.mode;

        let collector = self
            .parse_stream_oncer
            .as_ref()
            .repeat_exact(minimum)
            .collector(collector)
            .interspersed(int.as_ref())
            .parse_stream_once(input)?;

        Ok(int
            .as_ref()
            .then(self.parse_stream_oncer.as_ref())
            .repeat_eoi()
            .collector(collector)
            .parse_stream_once(input)?)
    }
}

impl<Par, Mod, Col> Repeater<Par, Mod, Col> {
    #[inline(always)]
    pub fn interspersed<Int>(self, parse_stream_oncer: Int) -> Repeater<Par, Inter<Mod, Int>, Col> {
        Repeater {
            parse_stream_oncer: self.parse_stream_oncer,
            mode: Inter(self.mode, parse_stream_oncer),
            collector: self.collector,
        }
    }

    #[inline(always)]
    pub fn collector<T>(self, collector: T) -> Repeater<Par, Mod, T> {
        Repeater {
            parse_stream_oncer: self.parse_stream_oncer,
            mode: self.mode,
            collector,
        }
    }

    #[inline(always)]
    pub fn collect<T>(self) -> Repeater<Par, Mod, T>
    where
        T: Default,
    {
        Repeater {
            parse_stream_oncer: self.parse_stream_oncer,
            mode: self.mode,
            collector: T::default(),
        }
    }

    #[inline(always)]
    pub fn to_vec<Inp>(self) -> Repeater<Par, Mod, Vec<Par::Output>>
    where
        Par: ParserOnce,
    {
        Repeater {
            parse_stream_oncer: self.parse_stream_oncer,
            mode: self.mode,
            collector: vec![],
        }
    }
}

impl<Par, Col> Repeater<Par, UntilErr, Col> {
    #[inline(always)]
    pub fn eoi(self) -> Repeater<Par, UntilEOI, Col> {
        Repeater {
            parse_stream_oncer: self.parse_stream_oncer,
            mode: UntilEOI,
            collector: self.collector,
        }
    }
}

impl<Par, Col> Repeater<Par, Minimum, Col> {
    #[inline(always)]
    pub fn eoi(self) -> Repeater<Par, MinimumEOI, Col> {
        Repeater {
            parse_stream_oncer: self.parse_stream_oncer,
            mode: MinimumEOI(self.mode.0),
            collector: self.collector,
        }
    }
}

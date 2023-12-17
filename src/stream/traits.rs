use crate::parser::traits::{Parse, Parser};

pub trait Stream {
    type Item;
    type Offset;
    type Span;
    type Peek<'a>
    where
        Self: 'a;
    type Slice<'a>
    where
        Self: 'a;

    fn offset(&self) -> Self::Offset;
    fn offset_mut(&mut self) -> &mut Self::Offset;

    fn skip(&mut self);
    fn advance(&mut self, offset: Self::Offset);

    fn retract(&mut self);
    fn go_back(&mut self, offset: Self::Offset);

    fn nth(&mut self, offset: Self::Offset) -> Option<Self::Item>;
    fn peek_nth(&self, offset: Self::Offset) -> Option<Self::Peek<'_>>;

    fn slice(&self, start: Self::Offset, end: Self::Offset) -> Self::Slice<'_>;
    fn span(&self, start: Self::Offset, end: Self::Offset) -> Self::Span;

    fn next(&mut self) -> Option<Self::Item> {
        self.nth(self.offset())
    }

    fn peek(&self) -> Option<Self::Peek<'_>> {
        self.peek_nth(self.offset())
    }

    fn parse<T>(&mut self) -> T
    where
        T: Parse<Input = Self>,
    {
        T::parse(self)
    }

    fn eval<Par>(&mut self, parser: Par) -> Par::Output
    where
        Self: Sized,
        Par: Parser<Self>,
    {
        parser.parse_stream(self)
    }

    fn has_next(&self) -> bool {
        match self.peek() {
            Some(_) => false,
            None => true,
        }
    }
}

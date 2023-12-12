use super::traits::Stream;

impl<T> Stream for (Vec<T>, usize)
where
    T: Clone,
{
    type Item = T;
    type Offset = usize;
    type Span = (usize, usize);
    type Peek<'a> = &'a T
    where
        Self: 'a;
    type Slice<'a> = &'a [T]
    where
        Self: 'a;

    fn offset(&self) -> Self::Offset {
        self.1
    }

    fn offset_mut(&mut self) -> &mut Self::Offset {
        &mut self.1
    }

    fn skip(&mut self) {
        self.advance(1);
    }

    fn advance(&mut self, offset: Self::Offset) {
        *self.offset_mut() += offset;
    }

    fn retract(&mut self) {
        self.go_back(1);
    }

    fn go_back(&mut self, offset: Self::Offset) {
        *self.offset_mut() -= offset;
    }

    fn nth(&mut self, offset: Self::Offset) -> Option<Self::Item> {
        self.0.get(offset).cloned()
    }

    fn peek_nth(&self, offset: Self::Offset) -> Option<Self::Peek<'_>> {
        self.0.get(offset)
    }

    fn slice(&self, start: Self::Offset, end: Self::Offset) -> Self::Slice<'_> {
        &self.0[start..end]
    }

    fn span(&self, start: Self::Offset, end: Self::Offset) -> Self::Span {
        (start, end)
    }
}

impl<'b> Stream for (&'b str, usize) {
    type Item = char;
    type Offset = usize;
    type Span = (usize, usize);
    type Peek<'a> = char
    where
        Self: 'a;
    type Slice<'a> = &'a str
    where
        Self: 'a;

    fn offset(&self) -> Self::Offset {
        self.1
    }

    fn offset_mut(&mut self) -> &mut Self::Offset {
        &mut self.1
    }

    fn skip(&mut self) {
        self.advance(1);
    }

    fn advance(&mut self, offset: Self::Offset) {
        *self.offset_mut() += offset;
    }

    fn retract(&mut self) {
        self.go_back(1);
    }

    fn go_back(&mut self, offset: Self::Offset) {
        *self.offset_mut() -= offset;
    }

    fn nth(&mut self, offset: Self::Offset) -> Option<Self::Item> {
        self.0.chars().nth(offset)
    }

    fn peek_nth(&self, offset: Self::Offset) -> Option<Self::Peek<'_>> {
        self.0.chars().nth(offset)
    }

    fn slice(&self, start: Self::Offset, end: Self::Offset) -> Self::Slice<'_> {
        &self.0[start..end]
    }

    fn span(&self, start: Self::Offset, end: Self::Offset) -> Self::Span {
        (start, end)
    }
}

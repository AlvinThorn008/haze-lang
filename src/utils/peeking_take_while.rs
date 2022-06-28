use std::fmt::{Debug, self};
use std::iter::Peekable;

pub struct PeekingTakeWhile<I, P> {
    iter: I,
    predicate: P
}

impl<I, P> fmt::Debug for PeekingTakeWhile<I, P>
where
    I: Iterator + Debug,
    <I as Iterator>::Item: Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PeekingTakeWhile")
            .field("iter", &self.iter)
            .finish()
    }
}

impl<I, P> Iterator for PeekingTakeWhile<Peekable<I>, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_if(|x| (self.predicate)(x))
        
    }
}

impl<'s, I, P> Iterator for PeekingTakeWhile<&'s mut Peekable<I>, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_if(|x| (self.predicate)(x))
        
    }
}


pub trait PeekingTakeWhileExt: Iterator {
    fn peeking_take_while<P>(self, predicate: P) -> PeekingTakeWhile<Self, P>
    where
        Self: Iterator,
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        PeekingTakeWhile {
            iter: self,
            predicate,
        }
    }
}

impl<I: Iterator> PeekingTakeWhileExt for Peekable<I> {}

impl<I: Iterator> PeekingTakeWhileExt for &'_ mut Peekable<I> {}
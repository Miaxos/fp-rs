use crate::monad::Monad;
use futures::{Future, FutureExt};

/// Task<A> represents an asynchronous computation that yields a value of type A and never fails. If you want to represent an asynchronous computation that may fail, please see TaskEither.
pub trait Task<'a, A, B>: Monad<'a, A, B> + Future {
    fn delay() -> ();
}

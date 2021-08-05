use crate::applicative::Applicative;
use crate::chain::Chain;

/// The Monad type class combines the operations of the Chain and Applicative type classes. Therefore, Monad instances represent type constructors which support sequential composition, and also lifting of functions of arbitrary arity.
///
/// Instances must satisfy the following laws in addition to the Applicative and Chain laws:
///
/// Left identity: M.chain(M.of(a), f) <-> f(a)
/// Right identity: M.chain(fa, M.of) <-> fa
/// Note. Functor’s map can be derived: A.map = (fa, f) => A.chain(fa, a => A.of(f(a)))
pub trait Monad<'a, A, B>: Chain<'a, B> + Applicative<'a, A, B> {}

impl<'a, A, B> Monad<'a, A, B> for Option<A> {}

use crate::hkt::HKT;

/// The Chain type class SHOULD extends the Apply type class (but right now doesn't) with a chain operation which composes computations in sequence, using the return value of one computation to determine the next computation.
///
/// Instances must satisfy the following law in addition to the Apply laws:
/// Associativity: F.chain(F.chain(fa, afb), bfc) <-> F.chain(fa, a => F.chain(afb(a), bfc))
///
/// Note. Apply’s ap can be derived: (fab, fa) => F.chain(fab, f => F.map(fa, f))
pub trait Chain<'t, F, B>: HKT<B> {
    fn chain<G>(self, f: G) -> <Self as HKT<B>>::Target<'t>
    where
        G: FnOnce(<Self as HKT<B>>::Current<'t>) -> <Self as HKT<B>>::Target<'t>;
}

impl<'t, F, A, B> Chain<'t, F, B> for Option<A> {
    fn chain<G>(self, f: G) -> <Self as HKT<B>>::Target<'t>
    where
        G: FnOnce(<Self as HKT<B>>::Current<'t>) -> <Self as HKT<B>>::Target<'t>,
    {
        self.and_then(f)
    }
}

/*
impl<A, B, E> Chain<B> for Result<A, E> {
    fn chain<F>(self, f: F) -> Self::Target
    where
        F: FnOnce(A) -> <Self as HKT<B>>::Target,
    {
        self.and_then(f)
    }
}
*/

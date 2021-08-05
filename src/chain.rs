use crate::{apply::Apply, hkt::HKT};

/// The Chain type class extends the Apply type class (but right now doesn't) with a chain operation which composes computations in sequence, using the return value of one computation to determine the next computation.
///
/// Instances must satisfy the following law in addition to the Apply laws:
/// Associativity: F.chain(F.chain(fa, afb), bfc) <-> F.chain(fa, a => F.chain(afb(a), bfc))
///
/// Note. Apply’s ap can be derived: (fab, fa) => F.chain(fab, f => F.map(fa, f))
pub trait Chain<'t, B>: Apply<'t, B> {
    fn chain<G>(self, f: G) -> <Self as HKT<B>>::Target<'t>
    where
        G: FnOnce(<Self as HKT<B>>::Current<'t>) -> <Self as HKT<B>>::Target<'t>;
}

impl<'t, A, B> Chain<'t, B> for Option<A> {
    fn chain<G>(self, f: G) -> <Self as HKT<B>>::Target<'t>
    where
        G: FnOnce(<Self as HKT<B>>::Current<'t>) -> <Self as HKT<B>>::Target<'t>,
    {
        self.and_then(f)
    }
}

mod test {
    use super::Chain;

    #[test]
    fn test_chain() {
        let add_3 = |x: usize| Some(x + 3);
        let to_string = |x: usize| Some(format!("{}", x));
        let fa = Some(4).chain(add_3).chain(to_string);
        assert_eq!(fa, Some("7".to_string()));
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

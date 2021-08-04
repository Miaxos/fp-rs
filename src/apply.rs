use crate::functor::Functor;
use crate::hkt::HKT;

/// The `Apply` class provides the `ap` which is used to apply a function to an argument under a type constructor.
/// `Apply` can be used to lift functions of two or more arguments to work on values wrapped with the type constructor
///`f`.
///
/// Instances must satisfy the following law in addition to the `Functor` laws:
///     1. Associative composition: `F.ap(F.ap(F.fmap(fbc, bc => ab => a => bc(ab(a))), fab), fa) <-> F.ap(fbc, F.ap(fab, fa))`
///
/// Formally, `Apply` represents a strong lax semi-monoidal endofunctor.
pub trait Apply<'a, F, B>: Functor<'a, B> + HKT<F>
where
    F: FnOnce(<Self as HKT<B>>::Current<'a>) -> B,
{
    fn ap(self, f: <Self as HKT<F>>::Target<'a>) -> <Self as HKT<B>>::Target<'a>;
}

// Implementation for Option
//
impl<'a, A, F, B> Apply<'a, F, B> for Option<A>
where
    F: FnOnce(<Self as HKT<B>>::Current<'a>) -> B,
{
    fn ap(self, f: <Self as HKT<F>>::Target<'a>) -> <Self as HKT<B>>::Target<'a> {
        self.and_then(|v| f.fmap(|z| z(v)))
    }
}

mod test {
    use super::Apply;

    #[test]
    fn test_apply() {
        let fa = Some(1).ap(Some(|x| x + 1));
        assert_eq!(fa, Some(2));
    }
}

/*
impl<A, B, E> Apply<B> for Result<A, E> {
    fn ap(self, f: Applicator<B, Self>) -> <Self as HKT<B>>::Target {
        self.and_then(|v| f.map(|z| z(v)))
    }
}
*/

use crate::hkt::HKT;

///
/// A `Functor` is a type constructor which supports a mapping operation `map`.
///
/// `fmap`can be used to turn function `a -> b` into functions `f a -> f b` whose argument and
/// return types uses the type constructior `f`to represent some computational context.
///
/// The origin value is consumed by the fmap.
///
/// Must statisfy the following laws:
///
/// 1. Identity: F.fmap(fa, a => a) <-> fa`
/// 2. Composition: `F.fmap(fa, a => bc(ab(a))) <-> F.fmap(F.fmap(fa, ab), bc)`

pub trait Functor<'a, B>: HKT<B> {
    fn fmap<F>(self, f: F) -> Self::Target<'a>
    where
        F: FnOnce(Self::Current<'a>) -> B;
}

// Implementations should move elsewere after
impl<'t, A, B> Functor<'t, B> for Option<A> {
    fn fmap<F>(self, f: F) -> Self::Target<'t>
    where
        // A is Self::Current
        F: FnOnce(A) -> B,
    {
        self.map(f)
    }
}

/*
impl<'t, A, B, E> Functor<'t, B> for Result<A, E> {
    fn fmap<F>(self, f: F) -> Self::Target<'t>
    where
        // A is Self::Current
        F: FnOnce(A) -> B,
    {
        self.map(f)
    }
}
*/

mod test {
    use super::Functor;

    #[test]
    fn test_functor() {
        let truc = Some(1);
        let macin = truc.fmap(|x| x.to_string()).fmap(|x| format!("{}2", x));
        assert_eq!(macin, Some("12".to_string()));
    }
}

use crate::hkt::HKT;

pub trait Pointed<'t, F>: HKT<F> {
    fn of(value: Self::Current<'t>) -> Self::Target<'t>;
}

// Impl for options
impl<'t, A> Pointed<'t, A> for Option<A> {
    fn of(a: A) -> Self::Target<'t> {
        Some(a)
    }
}

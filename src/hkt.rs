//! Type defunctionalization (as describe in [Lightweight higher-kinded polymorphism](https://www.cl.cam.ac.uk/~jdy22/papers/lightweight-higher-kinded-polymorphism.pdf))

/// `* -> *` constructors
pub trait HKT<URI> {
    type URI<'u>;
    type Current<'c>;
    type Target<'t>;
}

/// `* -> *` constructors
pub trait HKT2<URI> {
    type URI<'u>;
    type Current<'c>;
    type Error<'e>;
    type Target<'t>;
}

macro_rules! derive_hkt {
    ($t: ident) => {
        impl<T, U> HKT<U> for $t<T> {
            type URI<'u> = U;
            type Current<'c> = T;
            type Target<'t> = $t<U>;
        }
    };
}

macro_rules! derive_hkt2 {
    ($t: ident) => {
        impl<T, U, E> HKT2<U> for $t<T, E> {
            type URI<'u> = U;
            type Current<'c> = T;
            type Error<'e> = E;
            type Target<'t> = $t<U, E>;
        }
    };
}

derive_hkt!(Option);
derive_hkt!(Vec);
derive_hkt2!(Result);

/*
pub trait HKT3<U1, U2> {
    type Current1<'c1>;
    type Current2<'c2>;
    type Target<'t>;
}

macro_rules! derive_hkt3 {
    ($t:ident) => {
        impl<T1, T2, U1, U2> HKT3<U1, U2> for $t<T1, T2> {
            // The currently contained types
            type Current1<'c1> = T1;
            type Current2<'c2> = T2;
            // How the U's get filled in.
            type Target<'c1> = $t<U1, U2>;
        }
    };
}

impl<A, F, B> HKT3<F, B> for Option<A> {
    type Current1<'c1> = A;
    type Current2<'c> = F;
    type Target<'t> = Option<F>;
}

derive_hkt3!(HashMap);
*/

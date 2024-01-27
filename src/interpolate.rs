/// Trait for data that can be linearly interpolated using type T.
pub trait Interpolate<T> {
    fn interpolate(&self, other: &Self, a: T, b: T) -> Self;
}

macro_rules! impl_interpolate {
    // The `tt` (token tree) designator is used for
    // operators and tokens.
    ($t:ty) => {
        impl Interpolate<$t> for $t {
            #[inline]
            fn interpolate(&self, other: &Self, a: $t, b: $t) -> Self {
                let x = a / (a - b);
                (1. - x) * *self + x * *other
            }
        }
    };
}

impl_interpolate!(f32);
impl_interpolate!(f64);

impl<const N: usize, T: Copy, U: Interpolate<T>> Interpolate<T> for [U; N]
where
    [U; N]: Default,
{
    fn interpolate(&self, other: &Self, a: T, b: T) -> Self {
        let mut r: [U; N] = Default::default();

        for ((x, y), r) in self.iter().zip(other).zip(&mut r) {
            *r = x.interpolate(y, a, b);
        }

        r
    }
}

impl<T> Interpolate<T> for () {
    fn interpolate(&self, _other: &Self, _a: T, _b: T) -> Self {
        ()
    }
}

impl<T, U, V> Interpolate<T> for (U, V)
where
    T: Copy,
    U: Interpolate<T>,
    V: Interpolate<T>,
{
    fn interpolate(&self, other: &Self, a: T, b: T) -> Self {
        (
            self.0.interpolate(&other.0, a, b),
            self.1.interpolate(&other.1, a, b),
        )
    }
}

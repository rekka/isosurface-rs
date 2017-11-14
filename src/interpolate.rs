/// Trait for data that can be linearly interpolated using type T.
pub trait Interpolate<T> {
    fn interpolate(&self, other: &Self, a: T, b: T) -> Self;
}

impl Interpolate<f64> for f64 {
    #[inline]
    fn interpolate(&self, other: &Self, a: f64, b: f64) -> Self {
        let x = a / (a - b);
        (1. - x) * *self + x * *other
    }
}

impl Interpolate<f64> for [f64; 2] {
    #[inline]
    fn interpolate(&self, other: &Self, a: f64, b: f64) -> Self {
        let x = a / (a - b);
        let v = self;
        let w = other;
        [(1. - x) * v[0] + x * w[0], (1. - x) * v[1] + x * w[1]]
    }
}

impl Interpolate<f64> for [f64; 3] {
    #[inline]
    fn interpolate(&self, other: &Self, a: f64, b: f64) -> Self {
        let x = a / (a - b);
        let v = self;
        let w = other;
        [(1. - x) * v[0] + x * w[0], (1. - x) * v[1] + x * w[1], (1. - x) * v[2] + x * w[2]]
    }
}

impl<T> Interpolate<T> for () {
    fn interpolate(&self, _other: &Self, _a: T, _b: T) -> Self {
        ()
    }
}

impl<T, U, V> Interpolate<T> for (U, V)
    where T: Copy,
          U: Interpolate<T>,
          V: Interpolate<T>
{
    fn interpolate(&self, other: &Self, a: T, b: T) -> Self {
        (self.0.interpolate(&other.0, a, b), self.1.interpolate(&other.1, a, b))
    }
}

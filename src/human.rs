// Axel '0vercl0k' Souchet - April 22 2024
use std::fmt::Display;

/// This trait adds convenient functions to display data for Humans. It is the
/// glue between the generic types [`HumanBytes<T>`].
pub trait ToHuman: Sized + Copy {
    fn human_bytes(&self) -> HumanBytes<Self> {
        HumanBytes(*self)
    }
}

/// Blanket implementation for all the `T` that have what we need.
impl<T> ToHuman for T
where
    T: TryInto<u64>,
    T: Copy,
{
}

// impl<T> ToHuman for Saturating<T>
// where
//     T: TryInto<u64>,
//     T: Copy,
// {
// }

/// Type that implements [`Display`] to print out a size in human form.
pub struct HumanBytes<T>(T);

impl<T> Display for HumanBytes<T>
where
    u64: TryFrom<T>,
    T: Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut unit = "b";
        // At worst, we get slightly incorrect results? :shrug:
        #[allow(clippy::cast_precision_loss)]
        let mut size = u64::try_from(self.0).map_err(|_| std::fmt::Error)? as f64;
        let k = 1_024f64;
        let m = k * k;
        let g = m * k;
        if size >= g {
            size /= g;
            unit = "gb";
        } else if size >= m {
            size /= m;
            unit = "mb";
        } else if size >= k {
            size /= k;
            unit = "kb";
        }

        write!(f, "{size:.1}{unit}")
    }
}

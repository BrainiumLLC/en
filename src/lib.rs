mod cast;
mod max;
mod min;

pub use self::{cast::*, max::Max, min::Min};
pub use num_traits;

use std::fmt::Debug;

pub trait Num:
    Copy + Debug + Max + Min + num_traits::Num + num_traits::NumCast + num_traits::NumRef
{
    fn two() -> Self {
        cast(2)
    }

    fn three() -> Self {
        cast(3)
    }

    fn four() -> Self {
        cast(4)
    }

    fn five() -> Self {
        cast(5)
    }

    fn six() -> Self {
        cast(6)
    }

    fn seven() -> Self {
        cast(7)
    }

    fn halved(self) -> Self {
        self / Self::two()
    }
}

impl<T> Num for T where
    T: Copy + Debug + Max + Min + num_traits::Num + num_traits::NumCast + num_traits::NumRef
{
}

pub trait Float: num_traits::Float + num_traits::FloatConst + Num {}
impl<T> Float for T where T: num_traits::Float + num_traits::FloatConst + Num {}

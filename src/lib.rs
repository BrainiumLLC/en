mod max;
mod min;

pub use self::{max::Max, min::Min};

use std::{any::type_name, fmt::Debug};

#[cold]
fn cast_num_fail<T, U: Debug>(value: U) -> ! {
    panic!(
        "cast failed: value {:?} of type `{}` could not be represented by type `{}`",
        value,
        type_name::<U>(),
        type_name::<T>(),
    )
}

pub fn cast<T: num_traits::NumCast, U: Copy + Debug + num_traits::ToPrimitive>(n: U) -> T {
    T::from(n).unwrap_or_else(move || cast_num_fail::<T, U>(n))
}

pub trait Num: Copy + Debug + Max + Min + num_traits::Num + num_traits::NumCast + num_traits::NumRef {
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

impl<T> Num for T where T: Copy + Debug + Max + Min + num_traits::Num + num_traits::NumCast + num_traits::NumRef {}

pub trait Float: num_traits::Float + num_traits::FloatConst + Num {}
impl<T> Float for T where T: num_traits::Float + num_traits::FloatConst + Num {}

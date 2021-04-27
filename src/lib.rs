mod cast;
mod max;
mod min;

pub use self::{cast::*, max::Max, min::Min};
pub use num_traits;

use std::fmt::Debug;

pub trait Num:
    Copy + Debug + Max + Min + num_traits::Num + num_traits::NumCast + num_traits::NumRef
{
    fn try_cast<T: num_traits::NumCast>(self) -> Result<T, CastFailure<T, Self>> {
        try_cast(self)
    }

    fn cast<T: num_traits::NumCast>(self) -> T {
        cast(self)
    }

    fn to_i8(self) -> i8 {
        self.cast()
    }

    fn to_i16(self) -> i16 {
        self.cast()
    }

    fn to_i32(self) -> i32 {
        self.cast()
    }

    fn to_i64(self) -> i64 {
        self.cast()
    }

    fn to_i128(self) -> i128 {
        self.cast()
    }

    fn to_isize(self) -> isize {
        self.cast()
    }

    fn to_u8(self) -> u8 {
        self.cast()
    }

    fn to_u16(self) -> u16 {
        self.cast()
    }

    fn to_u32(self) -> u32 {
        self.cast()
    }

    fn to_u64(self) -> u64 {
        self.cast()
    }

    fn to_u128(self) -> u128 {
        self.cast()
    }

    fn to_usize(self) -> usize {
        self.cast()
    }

    fn to_f32(self) -> f32 {
        self.cast()
    }

    fn to_f64(self) -> f64 {
        self.cast()
    }

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

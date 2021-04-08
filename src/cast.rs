use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct CastFailure<T, U: Debug> {
    value: U,
    _marker: std::marker::PhantomData<T>,
}

impl<T, U: Debug> From<U> for CastFailure<T, U> {
    fn from(value: U) -> Self {
        Self {
            value,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T, U: Debug> Display for CastFailure<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cast failed: value {:?} of type `{}` could not be represented by type `{}`",
            self.value,
            std::any::type_name::<U>(),
            std::any::type_name::<T>(),
        )
    }
}

impl<T, U: Debug> CastFailure<T, U> {
    #[cold]
    fn panic(self) -> ! {
        panic!("{}", self);
    }
}

pub fn try_cast<T: num_traits::NumCast, U: Copy + Debug + num_traits::ToPrimitive>(
    n: U,
) -> Result<T, CastFailure<T, U>> {
    T::from(n).ok_or_else(move || n.into())
}

pub fn cast<T: num_traits::NumCast, U: Copy + Debug + num_traits::ToPrimitive>(n: U) -> T {
    try_cast(n).unwrap_or_else(move |err| err.panic())
}

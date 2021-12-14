use crate::{EndFields, Transmutable};

macro_rules! impl_reflexively_transmutable {
    ($typ:ty) => {
        unsafe impl Transmutable<$typ> for $typ {}
    };
}

impl_reflexively_transmutable!(EndFields);

impl_reflexively_transmutable!(u8);
impl_reflexively_transmutable!(u16);
impl_reflexively_transmutable!(u32);
impl_reflexively_transmutable!(u64);
impl_reflexively_transmutable!(usize);
#[cfg(has_i128)]
impl_reflexively_transmutable!(u128);

impl_reflexively_transmutable!(i8);
impl_reflexively_transmutable!(i16);
impl_reflexively_transmutable!(i32);
impl_reflexively_transmutable!(i64);
impl_reflexively_transmutable!(isize);
#[cfg(has_i128)]
impl_reflexively_transmutable!(i128);

impl_reflexively_transmutable!(f32);
impl_reflexively_transmutable!(f64);

impl_reflexively_transmutable!(char);
impl_reflexively_transmutable!(bool);
impl_reflexively_transmutable!(());

unsafe impl<T, const N: usize> Transmutable<[T; N]> for [T; N] where T: Transmutable<T> {}

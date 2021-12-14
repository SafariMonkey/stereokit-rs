pub use ffi_helpers_macros::{FieldType, Repr, Transmutable};

pub unsafe trait Transmutable<T> {}

unsafe impl<T> Transmutable<T> for T {}

pub unsafe trait FieldType<const INDEX: usize> {
    type Type;
}

pub struct ReprC;
pub struct ReprTransparent;

pub unsafe trait Repr {
    type Repr;
}

mod sealed {
    pub struct  Sealed;
}

pub struct EndFields(sealed::Sealed);

#[cfg(test)]
mod tests {
    use crate::{FieldType, Repr, Transmutable};

    #[derive(Transmutable, Repr)]
    #[repr(transparent)]
    struct Basic {
        #[allow(dead_code)]
        foo: i32,
    }

    #[derive(FieldType, Repr)]
    #[repr(transparent)]
    struct SingleField<T>(T);

    static_assertions::assert_impl_all!(Basic: Transmutable<SingleField<i32>>);
    static_assertions::assert_not_impl_all!(Basic: Transmutable<SingleField<i64>>);
}

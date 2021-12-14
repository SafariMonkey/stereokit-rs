pub use ffi_helpers_macros::{FieldType, Transmutable};

pub unsafe trait Transmutable<T> {}

unsafe impl<T> Transmutable<T> for T {}

pub unsafe trait FieldType<const INDEX: usize> {
    type Type;
}

pub struct EndFields;

#[cfg(test)]
mod tests {
    use crate::{FieldType, Transmutable};

    #[derive(Transmutable)]
    struct Basic {
        #[allow(dead_code)]
        foo: i32,
    }

    #[derive(FieldType)]
    struct SingleField<T>(T);

    static_assertions::assert_impl_all!(Basic: Transmutable<SingleField<i32>>);
    static_assertions::assert_not_impl_all!(Basic: Transmutable<SingleField<i64>>);
}

use ffi_helpers::Transmutable;

pub unsafe trait Transmute<T> {
    fn transmute_ref_to(&self) -> &T;
    fn transmute_copy_to(&self) -> T;
}

unsafe impl<T, U> Transmute<U> for T
where
    T: Transmutable<U> + Copy,
    T: Sized,
    U: Sized,
{
    fn transmute_ref_to(&self) -> &U {
        unsafe { std::mem::transmute(self) }
    }
    fn transmute_copy_to(&self) -> U {
        unsafe { std::mem::transmute_copy(self) }
    }
}

pub unsafe trait TransmuteRev<T> {
    fn transmute_ref_from(&self) -> &T;
    fn transmute_copy_from(&self) -> T;
}

unsafe impl<T, U> TransmuteRev<U> for T
where
    U: Transmutable<T> + Copy,
    T: Sized,
    U: Sized,
{
    fn transmute_ref_from(&self) -> &U {
        unsafe { std::mem::transmute(self) }
    }
    fn transmute_copy_from(&self) -> U {
        unsafe { std::mem::transmute_copy(self) }
    }
}

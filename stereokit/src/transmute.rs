use ffi_helpers::Transmutable;

pub unsafe trait TransmuteCopy<T> {
    fn transmute_copy_to(&self) -> T;
}

unsafe impl<T, U> TransmuteCopy<U> for T
where
    T: Transmutable<U> + Copy,
    T: Sized,
    U: Sized,
{
    fn transmute_copy_to(&self) -> U {
        unsafe { std::mem::transmute_copy(self) }
    }
}

pub unsafe trait TransmuteCopyRev<T> {
    fn transmute_copy_from(&self) -> T;
}

unsafe impl<T, U> TransmuteCopyRev<U> for T
where
    U: Transmutable<T> + Copy,
    T: Sized,
    U: Sized,
{
    fn transmute_copy_from(&self) -> U {
        unsafe { std::mem::transmute_copy(self) }
    }
}

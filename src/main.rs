#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_const_unstable(feature = "const_ptr_write", issue = "86302")]
#[cfg_attr(miri, track_caller)]
pub const unsafe fn write<T>(dst: *mut T, src: T) {
    extern "rust-intrinsic" {
        #[rustc_const_stable(feature = "const_intrinsic_copy", since = "1.63.0")]
        fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize);
    }
    unsafe {
        copy_nonoverlapping(&src as *const T, dst, 1); // src에 대한 새로운 pointer 생성
        intrinsics::forget(src); // 여기서 forget하지 않으면 double-free
    }
}

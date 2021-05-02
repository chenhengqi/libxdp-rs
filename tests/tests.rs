#[cfg(test)]
mod tests {
    use libxdp_sys::*;

    unsafe extern "C" fn libxdp_print(
        _level: libxdp_print_level,
        _arg1: *const ::std::os::raw::c_char,
        _ap: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int {
        return 0;
    }

    #[test]
    fn test() {
        unsafe {
            assert!(libxdp_set_print(Some(libxdp_print as _)).is_some());
        }
    }
}

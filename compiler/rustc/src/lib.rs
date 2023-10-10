#![crate_type = "dylib"]
#[allow(dead_code)]
fn rust_hook() {
    // See the comment at the top of this file for an explanation of this.
    #[cfg(feature = "jemalloc-sys")]
    {
        use std::os::raw::{c_int, c_void};
         
        #[used]
        static _F1: unsafe extern "C" fn(usize, usize) -> *mut c_void = jemalloc_sys::calloc;
        #[used]
        static _F2: unsafe extern "C" fn(*mut *mut c_void, usize, usize) -> c_int =
            jemalloc_sys::posix_memalign;
        #[used]
        static _F3: unsafe extern "C" fn(usize, usize) -> *mut c_void = jemalloc_sys::aligned_alloc;
        #[used]
        static _F4: unsafe extern "C" fn(usize) -> *mut c_void = jemalloc_sys::malloc;
        #[used]
        static _F5: unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void = jemalloc_sys::realloc;
        #[used]
        static _F6: unsafe extern "C" fn(*mut c_void) = jemalloc_sys::free;

        // On OSX, jemalloc doesn't directly override malloc/free, but instead
        // registers itself with the allocator's zone APIs in a ctor. However,
        // the linker doesn't seem to consider ctors as "used" when statically
        // linking, so we need to explicitly depend on the function.
        #[cfg(target_os = "macos")]
        {
            extern "C" {
                fn _rjem_je_zone_register();
            }

            #[used]
            static _F7: unsafe extern "C" fn() = _rjem_je_zone_register;
        }
    }   
            
    rustc_driver::main()
}

#![feature(libc)]
extern crate libc;

#[derive(Debug)]
enum FzContext {}
enum FzAllocContext {}
enum FzLocksContext {}

#[link(name = "mupdf")]
#[link(name = "freetype")]
#[link(name = "png")]
#[link(name = "z")]
#[link(name = "jpeg")]
extern {
    fn fz_new_context_imp(alloc: *const FzAllocContext, locks: *const FzLocksContext, max_store: libc::c_uint, version: *const u8) -> *mut FzContext;
}

fn fz_new_context(alloc: *const FzAllocContext, locks: *const FzLocksContext, max_store: libc::c_uint) -> *mut FzContext {
    unsafe { fz_new_context_imp(alloc, locks, max_store, "1.7a\0".as_ptr()) }
}

#[test]
fn it_works() {
    let ctx = fz_new_context(std::ptr::null(), std::ptr::null(), 0);
    println!("a pointer: {:?}", ctx)
}

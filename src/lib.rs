#![feature(libc)]
extern crate libc;

#[derive(Debug)]
pub enum FzContext {}
pub enum FzAllocContext {}
pub enum FzLocksContext {}

#[allow(dead_code)]
#[link(name = "mupdf")]
#[link(name = "freetype")]
#[link(name = "png")]
#[link(name = "z")]
#[link(name = "jpeg")]
extern {
    fn fz_new_context_imp(alloc: *const FzAllocContext, locks: *const FzLocksContext, max_store: libc::c_uint, version: *const u8) -> *mut FzContext;

    fn rust_mupdf_FZ_VERSION() -> *const u8;
}

fn fz_new_context(alloc: *const FzAllocContext, locks: *const FzLocksContext, max_store: libc::c_uint) -> *mut FzContext {
    unsafe { fz_new_context_imp(alloc, locks, max_store, rust_mupdf_FZ_VERSION()) }
}

#[test]
fn it_works() {
    let ctx = fz_new_context(std::ptr::null(), std::ptr::null(), 0);
    println!("ctx: {:?}", ctx)
}

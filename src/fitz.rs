extern crate std;
extern crate libc;

#[derive(Debug)]
pub enum FzContext {}
pub enum FzAllocContext {}
pub enum FzLocksContext {}

pub enum FzDocument {}
pub enum FzOutline {}
pub enum FzOutput {}
pub enum FzPage {}
pub enum FzTextPage {}
pub enum FzTextSheet {}
pub enum FzDevice {}
pub enum FzMatrix {}
pub enum FzCookie {}

pub enum FzRect {}
pub enum FzTransition {}
pub enum FzLink {}

#[allow(dead_code)]
#[link(name = "mupdf")]
#[link(name = "freetype")]
#[link(name = "png")]
#[link(name = "z")]
#[link(name = "jpeg")]
#[link(name = "jbig2dec")]
#[link(name = "crypto")]
#[link(name = "mujs")]
#[link(name = "openjp2")]
extern {
    fn fz_new_context_imp(alloc: *const FzAllocContext, locks: *const FzLocksContext, max_store: libc::c_uint, version: *const u8) -> *mut FzContext;
    fn fz_free_context(ctx: *mut FzContext);

    static fz_identity: FzMatrix;

    // mupdf/fitz/document.h

    fn fz_register_document_handlers(ctx: *mut FzContext);

    fn fz_open_document(ctx: *mut FzContext, filename: *const u8) -> *mut FzDocument;

    fn fz_count_pages(ctx: *mut FzContext, doc: *mut FzDocument) -> libc::c_int;

    fn fz_load_outline(ctx: *mut FzContext, doc: *mut FzDocument) -> *mut FzOutline;

    fn fz_load_page(ctx: *mut FzContext, doc: *mut FzDocument, number: libc::c_int) -> *mut FzPage;

    // mupdf/fitz/structured-text.h

    fn fz_new_text_sheet(ctx: *mut FzContext) -> *mut FzTextSheet;

    fn fz_print_text_page(ctx: *mut FzContext, output: *mut FzOutput, page: *mut FzTextPage);

    fn fz_new_text_device(ctx: *mut FzContext, sheet: *mut FzTextSheet, page: *mut FzTextPage) -> *mut FzDevice;

    fn fz_new_text_page(ctx: *mut FzContext) -> *mut FzTextPage;

    fn fz_run_page(ctx: *mut FzContext, page: *mut FzPage, device: *mut FzDevice, transform: *const FzMatrix, cookie: *const FzCookie);

    // mupdf/fitz/outline.h

    fn fz_print_outline(ctx: *mut FzContext, output: *mut FzOutput, outline: *mut FzOutline);

    // mupdf/fitz/output.h

    fn fz_new_output_to_filename(ctx: *mut FzContext, filename: *const u8) -> *mut FzOutput;

    // Internal helpers
    fn rust_mupdf_FZ_VERSION() -> *const u8;
}

pub fn fz_new_context(alloc: *const FzAllocContext, locks: *const FzLocksContext, max_store: libc::c_uint) -> *mut FzContext {
    unsafe { fz_new_context_imp(alloc, locks, max_store, rust_mupdf_FZ_VERSION()) }
}

#[test]
fn it_works() {
    let ctx = fz_new_context(std::ptr::null(), std::ptr::null(), 0);
    println!("ctx: {:?}", ctx);

    unsafe { fz_register_document_handlers(ctx); }

    let doc = unsafe { fz_open_document(ctx, "test.pdf\0".as_ptr()) };
    let pagecount = unsafe { fz_count_pages(ctx, doc) };
    println!("pages: {}", pagecount);

    let outline = unsafe { fz_load_outline(ctx, doc) };
    let output = unsafe { fz_new_output_to_filename(ctx, "test.log\0".as_ptr()) };
    unsafe { fz_print_outline(ctx, output, outline); }

    let page = unsafe { fz_load_page(ctx, doc, 0) };

    let sheet = unsafe { fz_new_text_sheet(ctx) };
    let text_page = unsafe { fz_new_text_page(ctx) };
    let dev = unsafe { fz_new_text_device(ctx, sheet, text_page) };
    unsafe { fz_run_page(ctx, page, dev, &fz_identity, std::ptr::null()) }

    unsafe { fz_print_text_page(ctx, output, text_page) };
}

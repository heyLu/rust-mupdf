#![feature(libc)]
extern crate libc;

#[derive(Debug)]
pub enum FzContext {}
pub enum FzAllocContext {}
pub enum FzLocksContext {}

/*impl Drop for FzContext {
    fn drop(&mut self) {
        println!("dropping!");
        unsafe { fz_free_context(self) }
    }
}*/



/*#[repr(C)]
struct FzOutline {
    title: *mut u8,
    dest: FzLinkDest,
    next: *mut FzOutline,
    down: *mut FzOutline,
}*/

pub enum FzOutput {}

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
    fn fz_free_context(ctx: *const FzContext);

    fn pdf_open_document(ctx: *mut FzContext, filename: *const u8) -> *mut PDFDocument;

    fn fz_print_outline(ctx: *mut FzContext, out: *mut FzOutput, outline: *mut FzOutline);
    fn fz_new_output_to_filename(ctx: *mut FzContext, filename: *const u8) -> *mut FzOutput;
    fn pdf_load_outline(doc: *mut PDFDocument) -> *mut FzOutline;

    // Internal helpers
    fn rust_mupdf_FZ_VERSION() -> *const u8;
}

fn fz_new_context(alloc: *const FzAllocContext, locks: *const FzLocksContext, max_store: libc::c_uint) -> *mut FzContext {
    unsafe { fz_new_context_imp(alloc, locks, max_store, rust_mupdf_FZ_VERSION()) }
}

enum FzStream {}
enum PDFCrypt {}
enum PDFOCGDescriptor {}
enum PDFHotspot {}
enum PDFXref {}

enum FzOutline {}
enum FzPage {}
enum FzLink {}
enum FzRect {}
enum FzDevice {}
enum FzMatrix {}
enum FzCookie {}
enum FzAnnot {}
enum FzTransition {}
enum FzWriteOptions {}

#[repr(C)]
struct FzDocument {
    close: *mut extern "C" fn(*const FzDocument),
    needs_password: *mut extern "C" fn(*mut FzDocument) -> libc::c_int,
    authenticate_password: *mut extern "C" fn(*mut FzDocument, *const u8) -> libc::c_int,
    load_outline: *mut extern "C" fn(*mut FzDocument) -> *mut FzOutline,
    count_pages: *mut extern "C" fn(*mut FzDocument) -> libc::c_int,
    load_page: *mut extern "C" fn(*mut FzDocument, libc::c_int) -> *mut FzPage,
    load_links: *mut extern "C" fn(*mut FzDocument, *mut FzPage) -> *mut FzLink,
    bound_page: *mut extern "C" fn(*mut FzDocument, *mut FzPage, *mut FzRect) -> *mut FzRect,
    run_page_contents: *mut extern "C" fn(*mut FzDocument, *mut FzPage, *mut FzDevice, *const FzMatrix, *mut FzCookie),
    run_annot: *mut extern "C" fn(*mut FzDocument, *mut FzPage, *mut FzAnnot, *mut FzDevice, *const FzMatrix, *mut FzCookie),
    free_page: *mut extern "C" fn(*mut FzDocument, *mut FzPage),
    meta: *mut extern "C" fn(*mut FzDocument, libc::c_int, *mut libc::c_void, libc::c_int) -> libc::c_int,
    page_presentation: *mut extern "C" fn(*mut FzDocument, *mut FzPage, *mut libc::c_float) -> *mut FzTransition,
    first_annot: *mut extern "C" fn(*mut FzDocument, *mut FzPage) -> *mut FzAnnot,
    next_annot: *mut extern "C" fn(*mut FzDocument, *mut FzAnnot) -> *mut FzAnnot,
    bound_annot: *mut extern "C" fn(*mut FzDocument, *mut FzAnnot, *mut FzRect) -> *mut FzRect,
    write: *mut extern "C" fn(*mut FzDocument, *mut u8, *mut FzWriteOptions),
    rebind: *mut extern "C" fn(*mut FzDocument, *mut FzContext),
}

enum PDFObj {}

#[repr(C)]
struct PageHint {
    number: libc::c_int,
    offset: libc::c_int,
    index: libc::c_int,
}

#[repr(C)]
struct PageHintShared {
    number: libc::c_int,
    offset: libc::c_int,
}

enum PDFLexbufLarge {}
enum PDFAnnot {}
enum PDFJS {}
enum PDFDocEvent {}
enum FzFont {}

#[repr(C)]
struct PDFDocument {
    super_: FzDocument,

    ctx: *mut FzContext,
    file: *mut FzStream,

    version: libc::c_int,
    startxref: libc::c_int,
    file_size: libc::c_int,
    crypt: *mut PDFCrypt,
    ocg: *mut PDFOCGDescriptor,
    hotspot: PDFHotspot,

    num_xref_sections: libc::c_int,
    xref_sections: *mut PDFXref,
    xref_altered: libc::c_int,
    freeze_updates: libc::c_int,
    has_xref_streams: libc::c_int,

    page_count: libc::c_int,

    repair_attempted: libc::c_int,

    file_reading_linearly: libc::c_int,
    file_length: libc::c_int,

    linear_obj: *mut PDFObj,
    linear_page_refs: *mut *mut PDFObj,

    hint_object_offset: libc::c_int,
    hint_object_length: libc::c_int,
    hints_loaded: libc::c_int,
    hint_page: *mut PageHint,
    hint_shared_ref: *mut libc::c_int,
    hint_shared: *mut PageHintShared,

    resources_localised: libc::c_int,

    lexbuf: PDFLexbufLarge,

    focus: *mut PDFAnnot,
    focus_obj: *mut PDFObj,

    js: *mut PDFJS,
    drop_js: *mut extern "C" fn(*mut PDFJS),
    recalculating: libc::c_int,
    dirty: libc::c_int,
    unsaved_sigs: *mut libc::c_void,

    update_appearance: *mut extern "C" fn(*mut PDFDocument, *mut PDFAnnot),

    event_cb: *mut extern "C" fn(*mut PDFDocEvent, *mut libc::c_void),
    event_cb_data: *mut libc::c_void,

    num_type3_fonts: libc::c_int,
    max_type3_fonts: libc::c_int,
    type3_fonts: *mut *mut FzFont,
}

#[test]
fn it_works() {
    let ctx = fz_new_context(std::ptr::null(), std::ptr::null(), 0);
    println!("ctx: {:?}", ctx);

    let doc = unsafe { pdf_open_document(ctx, "test.pdf\0".as_ptr()) };
    println!("version: {}", unsafe { (*doc).version });
    println!("file_size: {}", unsafe { (*doc).file_size } );
    println!("page_count: {}", unsafe { (*doc).page_count });

    let output = unsafe { fz_new_output_to_filename(ctx, "test.log\0".as_ptr()) };
    let outline = unsafe { pdf_load_outline(doc) };
    unsafe { fz_print_outline(ctx, output, outline) };
}

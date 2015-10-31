extern crate std;
extern crate libc;

use fitz::{FzContext, FzMatrix, FzRect, FzTransition, FzPage, FzLink};

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
    fn pdf_open_document(ctx: *mut FzContext, filename: *const u8) -> *mut PDFDocument;

    fn pdf_count_pages(ctx: *mut FzContext, doc: *mut PDFDocument) -> libc::c_int;
    fn pdf_load_page(ctx: *mut FzContext, doc: *mut PDFDocument, number: libc::c_int) -> *mut PDFPage;
}

pub enum PDFDocument {}

pub enum PDFObj {}
pub enum PDFAnnot {}

#[repr(C)]
pub struct PDFPage {
    super_: FzPage,
    doc: *mut PDFDocument,

    ctm: FzMatrix,
    mediabox: FzRect,
    rotate: libc::c_int,
    transparency: libc::c_int,
    resources: *mut PDFObj,
    contents: *mut PDFObj,
    links: *mut FzLink,
    annots: *mut PDFAnnot,
    annot_tailp: *mut *mut PDFAnnot,
    changed_annots: *mut PDFAnnot,
    deleted_annots: *mut PDFAnnot,
    tmp_annots: *mut PDFAnnot,
    me: *mut PDFObj,
    duration: libc::c_float,
    transition_present: libc::c_int,
    transition: FzTransition,
    incomplete: libc::c_int,
}

#[test]
fn test_load_page() {
    let ctx = ::fitz::fz_new_context(std::ptr::null(), std::ptr::null(), 0);

    let doc = unsafe { pdf_open_document(ctx, "test.pdf\0".as_ptr()) };
    let num_pages = unsafe { pdf_count_pages(ctx, doc) };
    println!("pages: {}", num_pages);

    let page = unsafe { pdf_load_page(ctx, doc, 0) };
    println!("page rotate: {}", unsafe { (*page).rotate });
}

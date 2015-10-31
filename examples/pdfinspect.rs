extern crate mupdf;

use mupdf::*;

fn main() {
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

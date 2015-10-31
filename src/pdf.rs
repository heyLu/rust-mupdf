extern crate std;
extern crate libc;

use std::collections::HashMap;
use std::ffi;

use fitz::{FzContext};

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

    fn pdf_lookup_page_obj(ctx: *mut FzContext, doc: *mut PDFDocument, needle: libc::c_int) -> *mut PDFObj;

    fn pdf_is_null(ctx: *mut FzContext, obj: *mut PDFObj) -> libc::c_int;
    fn pdf_is_bool(ctx: *mut FzContext, obj: *mut PDFObj) -> libc::c_int;
    fn pdf_is_int(ctx: *mut FzContext, obj: *mut PDFObj) -> libc::c_int;
    fn pdf_is_real(ctx: *mut FzContext, obj: *mut PDFObj) -> libc::c_int;
    fn pdf_is_number(ctx: *mut FzContext, obj: *mut PDFObj) -> libc::c_int;
    fn pdf_is_name(ctx: *mut FzContext, obj: *mut PDFObj) -> libc::c_int;
    fn pdf_is_string(ctx: *mut FzContext, obj: *mut PDFObj) -> libc::c_int;
    fn pdf_is_array(ctx: *mut FzContext, obj: *mut PDFObj) -> libc::c_int;
    fn pdf_is_dict(ctx: *mut FzContext, obj: *mut PDFObj) -> libc::c_int;
    fn pdf_is_indirect(ctx: *mut FzContext, obj: *mut PDFObj) -> libc::c_int;
    fn pdf_is_stream(ctx: *mut FzContext, obj: *mut PDFObj) -> libc::c_int;

    fn pdf_to_bool(ctx: *mut FzContext, obj: *mut PDFObj) -> libc::c_int;
    fn pdf_to_int(ctx: *mut FzContext, obj: *mut PDFObj) -> libc::c_int;
    fn pdf_to_real(ctx: *mut FzContext, obj: *mut PDFObj) -> libc::c_float;
    fn pdf_to_name(ctx: *mut FzContext, obj: *mut PDFObj) -> *mut libc::c_char;

    fn pdf_array_len(ctx: *mut FzContext, array: *mut PDFObj) -> libc::c_int;
    fn pdf_array_get(ctx: *mut FzContext, array: *mut PDFObj, idx: libc::c_int) -> *mut PDFObj;

    fn pdf_dict_len(ctx: *mut FzContext, dict: *mut PDFObj) -> libc::c_int;
    fn pdf_dict_get_key(ctx: *mut FzContext, dict: *mut PDFObj, idx: libc::c_int) -> *mut PDFObj;
    fn pdf_dict_get_val(ctx: *mut FzContext, dict: *mut PDFObj, idx: libc::c_int) -> *mut PDFObj;
}

pub enum PDFDocument {}
pub enum PDFPage {}
pub enum PDFObj {}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum PDFObject {
    Null,
    Bool(bool),
    Int(i32),
    //Real(f32),
    Name(String),
    String(String),
    Array(Vec<PDFObject>),
    Dict(HashMap<String, PDFObject>),

    Unknown,
}

impl PDFObject {
    fn new(ctx: *mut FzContext, raw_obj: *mut PDFObj) -> PDFObject {
        if unsafe { pdf_is_null(ctx, raw_obj) } != 0 {
            PDFObject::Null
        } else if unsafe { pdf_is_bool(ctx, raw_obj) } != 0 {
            let i = unsafe { pdf_to_bool(ctx, raw_obj) };
            PDFObject::Bool(i != 0)
        } else if unsafe { pdf_is_int(ctx, raw_obj) } != 0 {
            println!("int");
            let i = unsafe { pdf_to_int(ctx, raw_obj) };
            PDFObject::Int(i)
        } else if unsafe { pdf_is_name(ctx, raw_obj) } != 0 {
            let n = unsafe { pdf_to_name(ctx, raw_obj) };
            // TODO: why does a segfault happen here?
            let s = unsafe { String::from_utf8(ffi::CString::from_raw(n).to_bytes().to_vec()).unwrap() };
            PDFObject::Name(s)
        } else if unsafe { pdf_is_array(ctx, raw_obj) } != 0 {
            let len = unsafe { pdf_array_len(ctx, raw_obj) } as usize;
            let mut vec = Vec::with_capacity(len);
            for i in 0..len {
                let raw_item = unsafe { pdf_array_get(ctx, raw_obj, i as libc::c_int) };
                vec.push(PDFObject::new(ctx, raw_item));
            }
            PDFObject::Array(vec)
        } else if unsafe { pdf_is_dict(ctx, raw_obj) } != 0 {
            let len = unsafe { pdf_dict_len(ctx, raw_obj) } as usize;
            let mut dict: HashMap<String, PDFObject> = HashMap::with_capacity(len);
            for i in 0..len {
                let raw_key = unsafe { pdf_dict_get_key(ctx, raw_obj, i as libc::c_int) };
                let key = PDFObject::new(ctx, raw_key);
                let raw_val = unsafe { pdf_dict_get_val(ctx, raw_obj, i as libc::c_int) };
                let val = PDFObject::new(ctx, raw_val);
                dict.insert(key.as_name().unwrap(), val);
            }
            PDFObject::Dict(dict)
        } else {
            PDFObject::Unknown
        }
    }

    fn as_bool(&self) -> Option<bool> {
        match self {
            &PDFObject::Bool(b) => Some(b),
            _ => None,
        }
    }

    fn as_int(&self) -> Option<i32> {
        match self {
            &PDFObject::Int(i) => Some(i),
            _ => None,
        }
    }

    fn as_name(&self) -> Option<String> {
        match self {
            &PDFObject::Name(ref s) => Some(s.clone()),
            _ => None,
        }
    }

    fn as_map(&self) -> Option<&HashMap<String, PDFObject>> {
        match self {
            &PDFObject::Dict(ref dict) => Some(dict),
            _ => None,
        }
    }
}

#[test]
fn test_load_page() {
    let ctx = ::fitz::fz_new_context(std::ptr::null(), std::ptr::null(), 0);

    let doc = unsafe { pdf_open_document(ctx, "test.pdf\0".as_ptr()) };
    let num_pages = unsafe { pdf_count_pages(ctx, doc) };
    println!("pages: {}", num_pages);

    let raw_page_obj = unsafe { pdf_lookup_page_obj(ctx, doc, 0) };
    let page_obj = PDFObject::new(ctx, raw_page_obj);
    println!("{:?}", page_obj);
}

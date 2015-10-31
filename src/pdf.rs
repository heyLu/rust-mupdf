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

    fn pdf_dict_len(ctx: *mut FzContext, dict: *mut PDFObj) -> libc::c_int;
    fn pdf_dict_get_key(ctx: *mut FzContext, dict: *mut PDFObj, idx: libc::c_int) -> *mut PDFObj;
    fn pdf_dict_get_val(ctx: *mut FzContext, dict: *mut PDFObj, idx: libc::c_int) -> *mut PDFObj;
}

pub enum PDFDocument {}
pub enum PDFPage {}
pub enum PDFObj {}

struct PDFObject {
    ctx: *mut FzContext,
    raw_obj: *mut PDFObj,
    type_: PDFObjectType,
}

impl PDFObject {
    fn new(ctx: *mut FzContext, raw_obj: *mut PDFObj) -> PDFObject {
        let type_ = if unsafe { pdf_is_null(ctx, raw_obj) } != 0 {
            PDFObjectType::Null
        } else if unsafe { pdf_is_bool(ctx, raw_obj) } != 0 {
            PDFObjectType::Bool
        } else if unsafe { pdf_is_int(ctx, raw_obj) } != 0 {
            PDFObjectType::Int
        } else if unsafe { pdf_is_real(ctx, raw_obj) } != 0 {
            PDFObjectType::Real
        } else if unsafe { pdf_is_name(ctx, raw_obj) } != 0 {
            PDFObjectType::Name
        } else if unsafe { pdf_is_string(ctx, raw_obj) } != 0 {
            PDFObjectType::String
        } else if unsafe { pdf_is_array(ctx, raw_obj) } != 0 {
            PDFObjectType::Array
        } else if unsafe { pdf_is_dict(ctx, raw_obj) } != 0 {
            PDFObjectType::Dict
        } else {
            PDFObjectType::Unknown
        };

        PDFObject {
            ctx: ctx,
            raw_obj: raw_obj,
            type_: type_,
        }
    }

    fn object_type(&self) -> &PDFObjectType {
        &self.type_
    }

    fn dict_len(&self) -> Option<i32> {
        match self.type_ {
            PDFObjectType::Dict => Some(unsafe { pdf_dict_len(self.ctx, self.raw_obj) } as i32),
            _ => None,
        }
    }

    fn dict_kvs(&self) -> Option<Vec<(PDFObject, PDFObject)>> {
        if self.type_ != PDFObjectType::Dict {
            return None
        }

        let len = self.dict_len().unwrap() as usize;
        let mut kvs: Vec<(PDFObject, PDFObject)> = Vec::with_capacity(len);
        for i in 0..len {
            let raw_key = unsafe { pdf_dict_get_key(self.ctx, self.raw_obj, i as libc::c_int) };
            let raw_val = unsafe { pdf_dict_get_val(self.ctx, self.raw_obj, i as libc::c_int) };
            kvs.push((PDFObject::new(self.ctx, raw_key), PDFObject::new(self.ctx, raw_val)));
        }
        Some(kvs)
    }
}

impl std::fmt::Debug for PDFObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.type_)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum PDFObjectType {
    Null,
    Bool,
    Int,
    Real,
    //Number,
    Name,
    String,
    Array,
    Dict,
    //Indirect,
    //Stream,

    Unknown,
}

#[test]
fn test_load_page() {
    let ctx = ::fitz::fz_new_context(std::ptr::null(), std::ptr::null(), 0);

    let doc = unsafe { pdf_open_document(ctx, "test.pdf\0".as_ptr()) };
    let num_pages = unsafe { pdf_count_pages(ctx, doc) };
    println!("pages: {}", num_pages);

    let raw_page_obj = unsafe { pdf_lookup_page_obj(ctx, doc, 0) };
    let page_obj = PDFObject::new(ctx, raw_page_obj);
    println!("{:?}", page_obj.object_type());

    println!("{:?}", page_obj.dict_len());
    for kv in page_obj.dict_kvs().unwrap().iter() {
        println!("{:?} -> {:?}", kv.0, kv.1);
    }
}

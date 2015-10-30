extern crate gcc;

fn main() {
    gcc::compile_library("librustmupdf_helpers.a", &["src/mupdf_macros.c"]);
}

use std::os::raw::{c_char, c_double, c_int, c_uint};

// FIXME: is this the correct way to get opaque types?
// FIXME: alternative: https://docs.rs/cairo-sys-rs/0.5.0/src/cairo_sys/lib.rs.html#64
// NOTE: https://github.com/rust-lang/rust/issues/27303
// NOTE: ask F/O about this
pub enum PopplerDocument {}
pub enum PopplerPage {}

pub const C_NULL: i32 = 1;

// FIXME: *const instead of mut pointers?

#[link(name = "poppler-glib")]
extern "C" {
    pub fn poppler_document_new_from_file(
        uri: *const c_char,
        password: *const c_char,
        error: *mut *mut glib::ffi::GError,
    ) -> *mut PopplerDocument;
    pub fn poppler_document_new_from_data(
        data: *mut c_char,
        length: c_int,
        password: *const c_char,
        error: *mut *mut glib::ffi::GError,
    ) -> *mut PopplerDocument;
    pub fn poppler_document_get_n_pages(document: *mut PopplerDocument) -> c_int;
    pub fn poppler_document_get_page(
        document: *mut PopplerDocument,
        index: c_int,
    ) -> *mut PopplerPage;

    pub fn poppler_document_get_title(document: *mut PopplerDocument) -> *mut c_char;
    pub fn poppler_document_get_metadata(document: *mut PopplerDocument) -> *mut c_char;
    pub fn poppler_document_get_pdf_version_string(document: *mut PopplerDocument) -> *mut c_char;
    pub fn poppler_document_get_permissions(document: *mut PopplerDocument) -> c_uint;

    pub fn poppler_page_get_size(
        page: *mut PopplerPage,
        width: *mut c_double,
        height: *mut c_double,
    );
    pub fn poppler_page_render(page: *mut PopplerPage, cairo: *mut cairo::ffi::cairo_t);
    pub fn poppler_page_render_for_printing(page: *mut PopplerPage, cairo: *mut cairo::ffi::cairo_t);

    pub fn poppler_page_get_text(page: *mut PopplerPage) -> *mut c_char;
}

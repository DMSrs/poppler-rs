use crate::util;
use crate::PopplerPage;
use poppler_sys::{poppler as sys, poppler_document as sys_doc};
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::path;

#[derive(Debug)]
pub struct PopplerDocument(*mut sys::PopplerDocument);

impl PopplerDocument {
    pub fn new_from_file<P: AsRef<path::Path>>(
        p: P,
        password: &str,
    ) -> Result<PopplerDocument, glib::error::Error> {
        let pw = CString::new(password).map_err(|_| {
            glib::error::Error::new(
                glib::FileError::Inval,
                "Password invalid (possibly contains NUL characters)",
            )
        })?;

        let path_cstring = util::path_to_glib_url(p)?;
        let doc = util::call_with_gerror(|err_ptr| unsafe {
            sys_doc::poppler_document_new_from_file(path_cstring.as_ptr(), pw.as_ptr(), err_ptr)
        })?;

        Ok(PopplerDocument(doc))
    }
    pub fn new_from_data(
        data: &mut [u8],
        password: &str,
    ) -> Result<PopplerDocument, glib::error::Error> {
        if data.len() == 0 {
            return Err(glib::error::Error::new(
                glib::FileError::Inval,
                "data is empty",
            ));
        }
        let pw = CString::new(password).map_err(|_| {
            glib::error::Error::new(
                glib::FileError::Inval,
                "Password invalid (possibly contains NUL characters)",
            )
        })?;

        let doc = util::call_with_gerror(|err_ptr| unsafe {
            sys_doc::poppler_document_new_from_data(
                data.as_mut_ptr() as *mut c_char,
                data.len() as c_int,
                pw.as_ptr(),
                err_ptr,
            )
        })?;

        Ok(PopplerDocument(doc))
    }
    pub fn get_title(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_char = sys_doc::poppler_document_get_title(self.0);
            if ptr.is_null() {
                None
            } else {
                CString::from_raw(ptr).into_string().ok()
            }
        }
    }
    pub fn get_metadata(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_char = sys_doc::poppler_document_get_metadata(self.0);
            if ptr.is_null() {
                None
            } else {
                CString::from_raw(ptr).into_string().ok()
            }
        }
    }
    pub fn get_pdf_version_string(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_char = sys_doc::poppler_document_get_pdf_version_string(self.0);
            if ptr.is_null() {
                None
            } else {
                CString::from_raw(ptr).into_string().ok()
            }
        }
    }
    pub fn get_permissions(&self) -> u8 {
        unsafe { sys_doc::poppler_document_get_permissions(self.0) as u8 }
    }

    pub fn get_n_pages(&self) -> usize {
        // FIXME: what's the correct type here? can we assume a document
        //        has a positive number of pages?
        (unsafe { sys_doc::poppler_document_get_n_pages(self.0) }) as usize
    }

    pub fn get_page(&self, index: usize) -> Option<PopplerPage> {
        match unsafe { sys_doc::poppler_document_get_page(self.0, index as c_int) } {
            ptr if ptr.is_null() => None,
            ptr => Some(PopplerPage::new(ptr)),
        }
    }
}

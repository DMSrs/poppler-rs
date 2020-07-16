use glib;
use glib::translate::from_glib_full;
use glib_sys::GError;
use std::ffi::{CString, OsString};
use std::{fs, path, ptr};

pub fn call_with_gerror<T, F>(f: F) -> Result<*mut T, glib::error::Error>
where
    F: FnOnce(*mut *mut GError) -> *mut T,
{
    // initialize error to a null-pointer
    let mut err = ptr::null_mut();

    // call the c-library function
    let return_value = f(&mut err as *mut *mut GError);

    if return_value.is_null() {
        // Safety: No idea whatsoever
        // XXX: This is a complete guess based on seeing similar-ish patterns in the glib codebase
        Err(unsafe { from_glib_full(err) })
    } else {
        Ok(return_value)
    }
}

pub fn path_to_glib_url<P: AsRef<path::Path>>(p: P) -> Result<CString, glib::error::Error> {
    // canonicalize path, try to wrap failures into a glib error
    let canonical = fs::canonicalize(p).map_err(|_| {
        glib::error::Error::new(
            glib::FileError::Noent,
            "Could not turn path into canonical path. Maybe it does not exist?",
        )
    })?;

    // construct path string
    let mut osstr_path: OsString = "file:///".into();
    osstr_path.push(canonical);

    // we need to round-trip to string, as not all os strings are 8 bytes
    let pdf_string = osstr_path.into_string().map_err(|_| {
        glib::error::Error::new(
            glib::FileError::Inval,
            "Path invalid (contains non-utf8 characters)",
        )
    })?;

    CString::new(pdf_string).map_err(|_| {
        glib::error::Error::new(
            glib::FileError::Inval,
            "Path invalid (contains NUL characters)",
        )
    })
}

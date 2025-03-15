#[allow(non_upper_case_globals, non_camel_case_types, dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::ffi::CString;

/// A safe wrapper around the C cups_connection.
pub struct CupsConnection {
    ptr: *mut bindings::cups_connection,
}

impl CupsConnection {
    /// Connects to a printer at the given URL.
    ///
    /// # Errors
    /// Returns an error if the URL is invalid, contains null bytes, or if the connection fails.
    pub fn connect(url: &str) -> Result<Self, Error> {
        let c_url = CString::new(url).map_err(|_| Error::InvalidInput)?;
        let mut status = 0;
        let ptr = unsafe { bindings::cups_connect(c_url.as_ptr(), &mut status) };

        if ptr.is_null() {
            match status {
                -1 => Err(Error::InvalidUrl),
                // Handle possible malloc failure (status remains 0)
                0 => Err(Error::Io(libc::ENOMEM)),
                _ => Err(Error::Io(status)),
            }
        } else {
            Ok(Self { ptr })
        }
    }

    /// Prints the provided data to the connected printer.
    ///
    /// # Errors
    /// Returns an error if the data contains null bytes or if an IO error occurs during printing.
    pub fn print(&self, data: &str) -> Result<(), Error> {
        let c_data = CString::new(data).map_err(|_| Error::InvalidInput)?;
        let result = unsafe { bindings::cups_print(self.ptr, c_data.as_ptr()) };

        if result == 0 {
            Ok(())
        } else {
            Err(Error::Io(result))
        }
    }
}

impl Drop for CupsConnection {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { bindings::cups_free(self.ptr) };
        }
    }
}

/// Error types that can occur during connection or printing.
#[derive(Debug)]
pub enum Error {
    InvalidInput,
    InvalidUrl,
    Io(i32),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidInput => write!(f, "Input contains null bytes"),
            Error::InvalidUrl => write!(f, "Invalid URL provided"),
            Error::Io(code) => write!(f, "IO error occurred: {}", code),
        }
    }
}

impl std::error::Error for Error {}

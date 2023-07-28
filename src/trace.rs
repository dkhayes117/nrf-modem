use core::ops::Neg;
use crate::Error;
use crate::Error::NrfError;
use crate::ffi::get_last_error;
/// Get trace data.
///
/// The application shall call this function to receive trace data from the modem.
pub fn trace_get(buffer: &mut [u8], timeout: i32) -> Result<usize, Error> {
    let mut result = unsafe {
        nrfxlib_sys::nrf_modem_trace_get(
            buffer.as_ptr() as *mut _,
            buffer.len() as *mut _,
            timeout
        ) as isize
    };

    // const NRF_EINPROGRESS: isize = -(nrfxlib_sys::NRF_EINPROGRESS as isize);
    // const NRF_ENODATA : isize = -(nrfxlib_sys::NRF_ENODATA as isize);
    // const NRF_ESHUTDOWN: isize = -(nrfxlib_sys::NRF_ESHUTDOWN as isize);
    // const NRF_EAGAIN: isize = -(nrfxlib_sys::NRF_EAGAIN as isize);
    // const NRF_ENOTSUP : isize = -(nrfxlib_sys::NRF_ENOTSUP as isize);

    match result {
        0 => {
            let length = buffer[..].len();
            unsafe{ nrfxlib_sys::nrf_modem_trace_processed(length)};
            Ok(length)
        },
        error => Err(NrfError(error)),
    }
}


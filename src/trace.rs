use core::ops::Neg;
use crate::Error;
use crate::Error::NrfError;
use crate::ffi::get_last_error;

pub fn trace_get(buffer: &mut [u8], timeout: i32) -> Result<usize, Error> {
    let mut result = unsafe {
        nrfxlib_sys::nrf_modem_trace_get(
            buffer.as_ptr() as *mut _,
            buffer.len() as *mut _,
            timeout
        ) as isize
    };

    if result == -1 {
        result = get_last_error().abs().neg();
    }

    // const NRF_EINPROGRESS: isize = -(nrfxlib_sys::NRF_EINPROGRESS as isize);
    // const NRF_ENODATA : isize = -(nrfxlib_sys::NRF_ENODATA as isize);
    // const NRF_ESHUTDOWN: isize = -(nrfxlib_sys::NRF_ESHUTDOWN as isize);
    // const NRF_EAGAIN: isize = -(nrfxlib_sys::NRF_EAGAIN as isize);
    // const NRF_ENOTSUP : isize = -(nrfxlib_sys::NRF_ENOTSUP as isize);

    match result {
        bytes_received @ 0.. => {
            unsafe{ nrfxlib_sys::nrf_modem_trace_processed(bytes_received as usize)};
            Ok(bytes_received as usize)
        },
        error => Err(NrfError(error)),
    }
}


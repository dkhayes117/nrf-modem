use crate::Error;
use crate::Error::NrfError;
use nrfxlib_sys::{nrf_modem_trace_data, nrf_modem_trace_get, nrf_modem_trace_processed};

/// Get trace data.
///
/// The application shall call this function to receive trace data from the modem.
/// Trace data is automatically freed once returned from this function.
pub async fn trace_get(timeout: i32) -> Result<&'static [nrf_modem_trace_data], Error> {
    let mut frag_ptr = core::ptr::null_mut();
    let mut length: usize = 0;

    let result =
        unsafe { nrf_modem_trace_get(&mut frag_ptr as *mut _, &mut length as *mut _, timeout) }
            as isize;

    // // If frags or nfrags is null
    // const NRF_FAULT: isize = -(nrfxlib_sys::NRF_EINPROGRESS as isize);
    // // Trace is already in progress by the application
    // const NRF_EINPROGRESS: isize = -(nrfxlib_sys::NRF_EINPROGRESS as isize);
    // // No more trace data is available until the modem is restarted
    // const NRF_ENODATA: isize = -(nrfxlib_sys::NRF_ENODATA as isize);
    // // modem was shutdown
    // const NRF_ESHUTDOWN: isize = -(nrfxlib_sys::NRF_ESHUTDOWN as isize);
    // // Request has timed out
    // const NRF_EAGAIN: isize = -(nrfxlib_sys::NRF_EAGAIN as isize);
    // // Traces are disabled because trace region size is zero
    // const NRF_ENOTSUP : isize = -(nrfxlib_sys::NRF_ENOTSUP as isize);

    match result {
        0 => {
            #[cfg(feature = "defmt")]
            defmt::trace!("nrf_modem_trace_get successful");

            let frags = unsafe { core::slice::from_raw_parts(frag_ptr, length) };
            Ok(frags)
        }

        error => Err(NrfError(error)),
    }
}

/// Notify the Modem library that the application has completed processing of trace data.
///
/// The application shall call this function to let the Modem library free the trace memory.
/// It is the application’s responsibility to call this function with the total size of all
/// trace fragments received. If the data is processed in chunks, the function can be called
/// with the size that is processed since last call.
pub fn trace_processed(length: usize) -> Result<(), Error> {
    let result = unsafe { nrf_modem_trace_processed(length) } as isize;

    match result {
        0 => Ok(()),
        error => Err(NrfError(error)),
    }
}

use super::ffi;

pub use ffi::{
    buffer_blurring_t as BufferBlurring, buffer_line_16bpp_t as BufferLine16bpp,
    buffer_line_4bpp_t as BufferLine4bpp, buffer_text_t as BufferText,
};
#[cfg(feature = "jpeg")]
pub use ffi::{buffer_jpeg_t as BufferJpeg, buffer_jpeg_work_t as BufferJpegWork};

/// Returns a buffer for one line of 16bpp data
///
/// # Safety
///
/// This function is unsafe because the caller has to guarantee that he:
/// 1) frees the buffer after use
/// 2) doesn't use the buffer after it was freed
pub unsafe fn get_buffer_16bpp(idx: u16, clear: bool) -> &'static mut BufferLine16bpp {
    unsafe {
        let ptr = ffi::buffers_get_line_16bpp(idx, clear);
        unwrap!(ptr.as_mut())
    }
}

pub fn free_buffer_16bpp(buffer: &mut BufferLine16bpp) {
    unsafe {
        ffi::buffers_free_line_16bpp(buffer);
    }
}

/// Returns a buffer for one line of 4bpp data
///
/// # Safety
///
/// This function is unsafe because the caller has to guarantee that he:
/// 1) frees the buffer after use
/// 2) doesn't use the buffer after it was freed
pub unsafe fn get_buffer_4bpp(idx: u16, clear: bool) -> &'static mut BufferLine4bpp {
    unsafe {
        let ptr = ffi::buffers_get_line_4bpp(idx, clear);
        unwrap!(ptr.as_mut())
    }
}

pub fn free_buffer_4bpp(buffer: &mut BufferLine4bpp) {
    unsafe {
        ffi::buffers_free_line_4bpp(buffer);
    }
}

/// Returns a buffer for one line of text
///
/// # Safety
///
/// This function is unsafe because the caller has to guarantee that he:
/// 1) frees the buffer after use
/// 2) doesn't use the buffer after it was freed
pub unsafe fn get_buffer_text(idx: u16, clear: bool) -> &'static mut BufferText {
    unsafe {
        let ptr = ffi::buffers_get_text(idx, clear);
        unwrap!(ptr.as_mut())
    }
}

pub fn free_buffer_text(buffer: &mut BufferText) {
    unsafe {
        ffi::buffers_free_text(buffer);
    }
}

/// Returns a buffer for jpeg data
///
/// # Safety
///
/// This function is unsafe because the caller has to guarantee that he:
/// 1) frees the buffer after use
/// 2) doesn't use the buffer after it was freed
#[cfg(feature = "jpeg")]
pub unsafe fn get_buffer_jpeg(idx: u16, clear: bool) -> &'static mut BufferJpeg {
    unsafe {
        let ptr = ffi::buffers_get_jpeg(idx, clear);
        unwrap!(ptr.as_mut())
    }
}

pub fn free_buffer_jpeg(buffer: &mut BufferJpeg) {
    unsafe {
        ffi::buffers_free_jpeg(buffer);
    }
}

/// Returns a jpeg work buffer
///
/// # Safety
///
/// This function is unsafe because the caller has to guarantee that he:
/// 1) frees the buffer after use
/// 2) doesn't use the buffer after it was freed
#[cfg(feature = "jpeg")]
pub unsafe fn get_buffer_jpeg_work(idx: u16, clear: bool) -> &'static mut BufferJpegWork {
    unsafe {
        let ptr = ffi::buffers_get_jpeg_work(idx, clear);
        unwrap!(ptr.as_mut())
    }
}

pub fn free_buffer_jpeg_work(buffer: &mut BufferJpegWork) {
    unsafe {
        ffi::buffers_free_jpeg_work(buffer);
    }
}

/// Returns a buffer for blurring data
///
/// # Safety
///
/// This function is unsafe because the caller has to guarantee that he:
/// 1) frees the buffer after use
/// 2) doesn't use the buffer after it was freed
pub unsafe fn get_buffer_blurring(idx: u16, clear: bool) -> &'static mut BufferBlurring {
    unsafe {
        let ptr = ffi::buffers_get_blurring(idx, clear);
        unwrap!(ptr.as_mut())
    }
}

pub fn free_buffer_blurring(buffer: &mut BufferBlurring) {
    unsafe {
        ffi::buffers_free_blurring(buffer);
    }
}

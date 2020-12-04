use winit::window::Window;
use winit::platform::windows::WindowExtWindows;
use crate::window::{Gui, Pixel};
use winapi::um::winuser::{ReleaseDC, GetDC};
use winapi::um::wingdi::{CreateCompatibleDC, DeleteDC, RGBTRIPLE, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, RGBQUAD, CreateDIBSection, DIB_RGB_COLORS, SelectObject, SetMapMode, GetMapMode, BitBlt, SRCCOPY, BITMAP, GetCurrentObject, OBJ_BITMAP, GetObjectA};
use std::mem;
use winapi::ctypes::c_void;
use winapi::shared::windef::HBITMAP;

pub struct WindowsGui {
    hwnd: HWND,
    w_dc: WindowDC,
    bitmap: HBITMAP,
    buffer: *mut RGBTRIPLE,
}

impl Gui for WindowsGui {
    fn create(window: &Window) -> Self {
        let hwnd = HWND::new(window.hwnd() as *mut _);
        let dc = hwnd.get_dc();
        let (w, h) = window.inner_size().into();
        let mut buffer = std::ptr::null_mut();
        Self {
            hwnd,
            w_dc: dc,
            bitmap: create_di_buffer(w, h, &mut buffer),
            buffer
        }
    }

    fn draw(&mut self, width: usize, height: usize, pixels: &[Pixel]) {
        let buffer_slice = unsafe { std::slice::from_raw_parts_mut(self.buffer, width*height)  };
        buffer_slice.copy_from_slice(unsafe { std::slice::from_raw_parts(pixels.as_ptr() as _, pixels.len()) });

        let src = DC::new(self.w_dc.inner);
        let old = src.select_object(self.bitmap as *mut c_void);
        unsafe { SetMapMode(src.inner, GetMapMode(self.w_dc.inner)); };

        let res = unsafe { BitBlt(
            self.w_dc.inner,
            0,
            0,
            width as i32,
            height as i32,
            src.inner,
            0,
            0,
            SRCCOPY
        ) };
        assert_ne!(res, 0);

        src.select_object(old);
    }

    fn window_size(&self) -> (usize, usize) {
        let mut header: BITMAP = unsafe { mem::zeroed() };
        let h_bitmap = unsafe { GetCurrentObject(self.w_dc.inner, OBJ_BITMAP) };
        unsafe { GetObjectA(h_bitmap, std::mem::size_of::<BITMAP>() as i32, &mut header as *mut _ as _) };
        (header.bmWidth as usize, header.bmHeight as usize)
    }
}

struct HWND {
    inner: winapi::shared::windef::HWND
}

impl HWND {
    pub fn new(inner: winapi::shared::windef::HWND) -> Self {
        Self {
            inner
        }
    }

    pub fn get_dc(&self) -> WindowDC {
        WindowDC::new(self.inner, unsafe { GetDC(self.inner) })
    }
}

struct WindowDC {
    hwnd: winapi::shared::windef::HWND,
    inner: winapi::shared::windef::HDC
}

impl WindowDC {
    pub fn new(hwnd: winapi::shared::windef::HWND, inner: winapi::shared::windef::HDC) -> Self {
        WindowDC { hwnd, inner }
    }
    pub fn select_object(&self, object: *mut c_void) -> *mut c_void {
        unsafe { SelectObject(self.inner, object) }
    }
}

impl Drop for WindowDC {
    fn drop(&mut self) {
        unsafe { ReleaseDC(self.hwnd, self.inner) };
    }
}

struct DC {
    inner: winapi::shared::windef::HDC
}

impl DC {
    pub fn new(compatible: winapi::shared::windef::HDC) -> Self {
        DC { inner: unsafe { CreateCompatibleDC(compatible) } }
    }
    pub fn select_object(&self, object: *mut c_void) -> *mut c_void {
        unsafe { SelectObject(self.inner, object) }
    }
}

impl Drop for DC {
    fn drop(&mut self) {
        unsafe { DeleteDC(self.inner) };
    }
}

fn create_di_buffer(
    width: i32,
    height: i32,
    buffer: *mut *mut RGBTRIPLE
) -> HBITMAP {
    let info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height,
            biPlanes: 1,
            biBitCount: 24,
            biCompression: BI_RGB,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0
        },
        bmiColors: [RGBQUAD {
            rgbBlue: u8::max_value(),
            rgbGreen: u8::max_value(),
            rgbRed: u8::max_value(),
            rgbReserved: 0
        }]
    };
    unsafe {
        CreateDIBSection(
            std::ptr::null_mut(),
            &info,
            DIB_RGB_COLORS,
            buffer as *mut *mut _,
            std::ptr::null_mut(),
            0
        )
    }
}

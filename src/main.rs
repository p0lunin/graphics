use winit::window::WindowBuilder;
use winit::platform::windows::WindowExtWindows;
use winapi::um::winuser::{GetWindowDC, BeginPaint, PAINTSTRUCT, EndPaint, ReleaseDC, GetDC};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::{Event, WindowEvent};
use winit::dpi::{Size, PhysicalSize};
use winapi::um::wingdi::{CreateBitmap, CreateCompatibleDC, SelectObject, BitBlt, SRCCOPY, DeleteObject, DeleteDC, GetCurrentObject, OBJ_BITMAP, GetObjectA, BITMAP, BITMAPINFO, BITMAPINFOHEADER, RGBQUAD, BI_RGB, CreateDIBSection, DIB_RGB_COLORS, SetMapMode, GetMapMode};
use std::time::{Instant, Duration};
use winapi::shared::windef::HDC;
use std::mem::MaybeUninit;
use winapi::ctypes::c_ulong;
use std::mem;
use winapi::shared::windef::HBITMAP;

fn get_pixels(width: usize, height: usize) -> Vec<RGBQUAD> {
    vec![RGBQUAD {
        rgbBlue: u8::max_value(),
        rgbGreen: 0,
        rgbRed: 0,
        rgbReserved: 0
    }; height * width]
}

fn create_di_buffer(
    width: i32,
    height: i32,
    buffer: *mut *mut RGBQUAD
) -> HBITMAP {
    let info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height,
            biPlanes: 1,
            biBitCount: 32,
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

fn render(surface: HDC) {
    let mut pixels = get_pixels(512, 512);
    let mut buffer = std::ptr::null_mut();
    let bitmap = unsafe { create_di_buffer(512, 512, &mut buffer) };

    let buffer_slice = unsafe { std::slice::from_raw_parts_mut(buffer, 512*512)  };
    buffer_slice.copy_from_slice(pixels.as_slice());

    let src = unsafe { CreateCompatibleDC(surface) };
    let old = unsafe { SelectObject(surface, bitmap as *mut _) };

    unsafe { SetMapMode(src, GetMapMode(surface)); };

    let res = unsafe { BitBlt(
        surface,
        0,
        0,
        512,
        512,
        src,
        0,
        0,
        SRCCOPY
    ) };
    assert_ne!(res, 0);

    unsafe {
        //SelectObject(surface, old);
        DeleteDC(src);
    };
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(Size::Physical(PhysicalSize::new(512, 512)))
        .build(&event_loop)
        .unwrap();
    let hwnd = window.hwnd() as *mut _;

    let surface = unsafe { GetDC(hwnd) };

    let mut header: BITMAP = unsafe { mem::zeroed() };
    let h_bitmap = unsafe { GetCurrentObject(surface, OBJ_BITMAP) };
    unsafe { GetObjectA(h_bitmap, std::mem::size_of::<BITMAP>() as i32, &mut header as *mut _ as _) };
    println!("width: {}, height: {}", header.bmWidth, header.bmHeight);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(16));

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            },
            Event::MainEventsCleared => {
            },
            Event::RedrawRequested(_) => {
            },
            _ => ()
        };

        render(surface);
    });
    unsafe { ReleaseDC(hwnd, surface) };
}
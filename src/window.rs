use winit::window::Window;
use crate::Pixel;

mod windows;

pub trait Gui {
    fn create(window: &Window) -> Self;
    fn draw(&mut self, width: usize, height: usize, pixels: &[Pixel]);
    fn window_size(&self) -> (usize, usize);
}

pub fn get_gui(window: &Window) -> impl Gui {
    #[cfg(target_os = "windows")]
    windows::WindowsGui::create(window)
}

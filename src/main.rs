use winit::window::WindowBuilder;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::{Event, WindowEvent};
use winit::dpi::{Size, PhysicalSize};
use std::time::{Instant, Duration};
use std::mem::MaybeUninit;
use std::mem;
use some_graphics::{Pixel, get_gui, Gui};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(Size::Physical(PhysicalSize::new(512, 512)))
        .build(&event_loop)
        .unwrap();
    let mut gui = get_gui(&window);
    let (width, height) = gui.window_size();
    println!("width: {}, height: {}", width, height);

    let canvas = vec![Pixel {
        blue: u8::max_value(),
        green: 0,
        red: 0
    }; 512 * 512];

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

        gui.draw(512, 512,&canvas);
    });
}
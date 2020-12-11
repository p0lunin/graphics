use winit::window::WindowBuilder;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::{Event, WindowEvent};
use winit::dpi::{Size, PhysicalSize};
use std::time::{Instant, Duration};
use std::mem::MaybeUninit;
use std::mem;
use some_graphics::{Pixel, get_gui, Gui, trilinear_interpolation, Renderer, Camera, RawModel};
use some_graphics::storage::Storage;

fn make_cool(pixels: &mut [Pixel], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let y_f = y as f32;
            let x_f = x as f32;
            pixels[y*width+x] = Pixel {
                blue: (x_f/(width as f32) * (u8::max_value() as f32)) as u8,
                green: (y_f/(height as f32) * (u8::max_value() as f32)) as u8,
                red: u8::max_value()
            }
        }
    }
}

fn create_model() -> RawModel {
    use na::{Vector3, Point3};
    use some_graphics::{Triangle};

    RawModel::new(
        vec![
            Point3::new(1.0, -0.3, -0.3),
            Point3::new(1.0, 0.3, 0.0),
            Point3::new(1.0, -0.3, 0.3),
        ],
        vec![
            Triangle::new([0.into(), 1.into(), 2.into()])
        ]
    )
}

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

    let mut renderer = Renderer::new(512, 512);
    let camera = Camera::new(na::Point3::new(0.0, 0.0, 0.0), na::Point3::new(1.0, 0.0, 0.0));
    let model = create_model();
    let mut models = vec![model];
    //make_cool(&mut canvas, 512, 512);

    let mut fps = 0;
    let mut instant = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll/*(Instant::now() + Duration::from_millis(16))*/;

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

        renderer.render(&camera, models.as_slice());
        gui.draw(512, 512,renderer.canvas());
        do_some(&mut models);
        fps += 1;
        if instant.elapsed().as_secs() >= 1 {
            println!("fps: {}", (fps as f32)/(instant.elapsed().as_secs() as f32));
            fps = 0;
            instant = Instant::now();
        }
    });
}

fn do_some(models: &mut [RawModel]) {
    use some_graphics::storage::Storage;
    use na::{Matrix4, Vector3};

    let store = models[0].storage_mut();
    store.update_all(|point| {
        point[0] += 0.01;
    });
}

mod machine;
mod screen;

use std::time::{Duration, Instant};

use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, StartCause, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() -> Result<(), Error> {
    let mut machine = machine::CHIP8::new();
    let program = std::fs::read("roms/test_opcode.ch8").expect("Problem reading file!");
    machine.load_program(&program);

    let timer_length = Duration::new(1 / machine::CLOCK_SPEED, 0);

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(machine::SCREEN_SIZE_X * 16, machine::SCREEN_SIZE_Y * 16);
        WindowBuilder::new()
            .with_title("Tortilla")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(
            machine::SCREEN_SIZE_X,
            machine::SCREEN_SIZE_Y,
            surface_texture,
        )?
    };

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(_) => {
            if screen::redraw(&mut pixels, &machine.screen)
                .map_err(|e| println!("Error redrawing screen: {}", e))
                .is_err()
            {
                control_flow.set_exit();
                return;
            }
        }
        Event::NewEvents(StartCause::Init) => {
            control_flow.set_wait_until(Instant::now() + timer_length);
        }
        Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
            control_flow.set_wait_until(Instant::now() + timer_length);
            // main interpreter loop goes here
            let instruction = machine.fetch();
            if machine.decode_and_execute(instruction) {
                window.request_redraw();
            }
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => control_flow.set_exit(),
        _ => (),
    });
}

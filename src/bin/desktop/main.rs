use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::NamedKey,
    window::WindowBuilder
};


fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window_builder = WindowBuilder::new()
        .with_title("5 букв")
        .with_resizable(false)
        .with_maximized(false)
        .with_inner_size(LogicalSize::<i32>::from((300, 1200)))
        .with_min_inner_size(LogicalSize::<i32>::from((300, 1200)))
        .with_max_inner_size(LogicalSize::<i32>::from((300, 1200)))
        .with_transparent(true)
        .with_decorations(false);
    let window = window_builder.build(&event_loop).unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            },
            Event::AboutToWait => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                window.request_redraw();
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            },
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { event, ..},
                ..
            } => {
                match event.logical_key {
                    winit::keyboard::Key::Named(NamedKey::Escape) => elwt.exit(),
                    _ => ()
                }
            },
            _ => ()
        }
    }).unwrap();
}

use std::rc::Rc;
use std::sync::{Mutex};
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::Window;
use wry::WebView;

/// The ApplicationWindow structure, containing:
///
/// An event loop, window, webview
pub struct ApplicationWindow {
    pub event_loop: Option<EventLoop<()>>,
    #[allow(unused)]
    pub window: Window,
    pub webview: Option<WebView>,
}

/*
 * Implementation of the Application structure.
 * Contains the new() and run() functions.
 */
impl ApplicationWindow {
    /// the run() function basically
    /// Runs the event_loop
    pub fn run(mut self) {
        let event_loop = self.event_loop.take().expect("event_loop already taken");
        let app = Rc::new(Mutex::new(self));

        event_loop.run(move |event, _event_loop, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::Destroyed { .. },
                    ..
                }
                | Event::WindowEvent {
                    event: WindowEvent::CloseRequested { .. },
                    ..
                } => {
                    let mut lock = app.lock().unwrap();
                    lock.shutdown(control_flow);
                    drop(lock);
                }
                _ => {}
            }
        });
    }

    pub fn shutdown(&mut self, control_flow: &mut ControlFlow) {
        self.webview.take();

        *control_flow = ControlFlow::Exit;
    }
}

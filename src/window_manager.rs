use std::rc::Rc;

use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::{Window, WindowBuilder};
use wry::WebView;
use crate::webview_manager::WebviewApp;

/// The Application structure, containing:
///
/// An event loop, window, webview
struct Application {
    event_loop: Option<EventLoop<()>>,
    #[allow(unused)]
    window: Option<Window>,
    webview: Option<WebView>,
}

/*
 * Implementation of the Application structure.
 * Contains the new() and run() functions.
 */
impl Application {
    /// The new() function returns a newly made instance of
    /// the Application struct. this function also contains
    /// the whole functionality of how the Window behaves.
    pub fn new(event_loop: EventLoop<()>) -> Application {
        let window = WindowBuilder::new();

        // PLATFORM SPECIFIC:
        // We have to build the window with vbox if
        // The app is running under linux.
        #[cfg(target_os = "linux")]
        use tao::platform::unix::{WindowBuilderExtUnix, WindowExtUnix};
        #[cfg(target_os = "linux")]
        let window = window.with_default_vbox(true);

        let window = window
            .build(&event_loop)
            // # Safety
            //
            // Why should we give a fuck anyway
            // If our window couldn't be drew?
            .expect("Couldn't create the window");

        window.set_title("Rust Webview Application Codebase");

        let webview = WebviewApp::new().builder;

        #[cfg(target_os = "linux")]
        use wry::WebViewBuilderExtUnix;
        // PLATFORM SPECIFIC:
        // We have to build the window with GTK if
        // The app is running under linux.
        #[cfg(target_os = "linux")]
        let webview = webview.build_gtk(
            window
                .default_vbox()
                // # Safety
                //
                // Why should we give a fuck anyway
                // If our window couldn't be drew?
                .expect("Cannot use default_vbox in this environment"),
        );

        // PLATFORM SPECIFIC:
        #[cfg(not(target_os = "linux"))]
        let webview = webview.build(&window);

        Application {
            event_loop: Some(event_loop),
            window: Some(window),
            webview: Some(
                webview
                    // # Safety
                    //
                    // Why should we give a fuck anyway
                    // If our window couldn't be drew?
                    .expect("Couldn't build the webview"),
            ),
        }
    }
    /// the run() function basically
    /// Runs the event_loop
    pub fn run(mut self) {
        let event_loop = self.event_loop.take().expect("event_loop already taken");
        let app = Rc::new(self);

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
                    app.shutdown(control_flow);
                }
                _ => {}
            }
        });
    }

    pub fn shutdown(&self, control_flow: &mut ControlFlow) {
        *control_flow = ControlFlow::Exit;

        self.webview.as_ref().take();
    }
}

/// Init the window
pub fn init() {
    Application::new(EventLoop::new()).run();
}

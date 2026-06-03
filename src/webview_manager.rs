use std::borrow::Cow;
use std::thread;
use wry::http::header::CONTENT_TYPE;
use wry::{WebViewBuilder, http::Response};

use log::error;
use log::info;

use crate::assets_bundled_manager::ASSETS;

/// The Webview structure, containing a webview builder
pub struct WebviewApp {
    pub builder: WebViewBuilder<'static>,
}

/*
 * Cargo clippy suggests us to implement the
 * Default trait for some reason
 * so yeah
 */
impl Default for WebviewApp {
    fn default() -> Self {
        Self::new()
    }
}

/*
 * Implementation of the WebviewApp structure.
 * Contains the new() function.
 */
impl WebviewApp {
    /// The new() function returns a newly made instance of
    /// the WebviewApp struct. this function also contains
    /// the whole functionality of how the webview behaves.
    /// Inside the with_asynchronous_custom_protocol's method body,
    /// There's the whole logic behind how we handle requests
    pub fn new() -> WebviewApp {
        // Return the struct
        WebviewApp {
            builder: WebViewBuilder::new()
                // Turning off the devtools isn't nercessary
                // But maybe would prevent user from accessing
                // the frontend somehow.
                .with_devtools(false)
                // We're not building a browser are we?
                .with_back_forward_navigation_gestures(false)
                // The base URL that asynchronous_custom_protocol would be listening on
                // TODO: this may need tweaks on Android, maybe it should be:
                // http://app.client/assets_bundled/
                .with_url("app://client/assets_bundled/test.txt")
                .with_asynchronous_custom_protocol("app".into(),
                {
                    move |_id, request, responder| {
                        // We can't move twice, so we clone variables before moving it twice
                        // This has no performance cost for us because we're using the
                        // Automatically Reference Counted (ARC) type, which just
                        // adds refrences to a variable instead of cloning & copying
                        // yet another one.

                        thread::spawn(move || {
                            let path = request.uri().path();
                            info!("{path}");

                            // We unwrap the lock of variables here & then take refs

                            let response: String = match path {
                                // Here, we fall back to our assets
                                _ => {
                                    let assets = ASSETS.get()
                                        // # Safety
                                        //
                                        // I mean why should we give a fuck if app crashes at this point
                                        // if there's no ASSETS variable I wonder how the fuck
                                        // the application got initialized even at this point bro
                                        .expect("webview_manager.rs: Couldn't get ASSETS");

                                    match assets.get(path) {
                                        Some(s) => s.into(),
                                        _ => {
                                            error!("Unknown path: {path}");

                                            assets
                                                .get("/assets_bundled/test.txt")
                                                // # Safety
                                                //
                                                // So you're telling me that we have our ASSETS set,
                                                // But we don't have a fucking main page?
                                                .expect("webview_manager.rs: Couldn't get the default page, missing assets?")
                                                .into()
                                        }
                                    }
                                }
                            };

                            // Here we build the actual response
                            responder.respond(
                                Response::builder()
                                    .header(CONTENT_TYPE,
                                        // Not sure why but sometimes some platforms are so
                                        // Paranoid about the response type, so we should
                                        // Specify our response type
                                        match path {
                                        p if p.ends_with(".html") => "text/html; charset=utf-8",
                                        p if p.ends_with(".css") => "text/css; charset=utf-8",
                                        p if p.ends_with(".html") => "application/javascript; charset=utf-8",
                                        _ => "text/plain; charset=utf-8",
                                    })
                                    .body(Cow::<[u8]>::Owned(response.as_bytes().to_vec()))
                                    // Whatever at this point bro...
                                    .unwrap_or_else(|_e| Response::default())
                            );
                        });
                    }
                }),
        }
    }
}

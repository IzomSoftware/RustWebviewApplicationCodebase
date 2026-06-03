use std::borrow::Cow;
// use std::thread;
use wry::http::header::CONTENT_TYPE;
use wry::{WebViewBuilder, http::Response};

use log::error;
use log::info;

use crate::app::assets_bundling::ASSETS;

/// The Webview structure, containing a webview builder
pub struct WebviewApp {
    pub builder: WebViewBuilder<'static>,
}

/*
 * Cargo clippy suggests us to implement the
 * Default trait for some reason
 */
impl Default for WebviewApp {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_response(path: String) -> &'static [u8] {
        match path {
            // Here, we fall back to our assets
            _ => {
                let assets = &ASSETS;

                match assets.get(&path) {
                    Some(s) => s,
                    _ => {
                        error!("Unknown path: {path}");

                        assets
                            .get("/assets_bundled/test.txt")
                            // # Safety
                            //
                            // So you're telling me that we have our ASSETS set,
                            // But we don't have a main page?
                            .expect("Couldn't get the default page, missing assets?")
                    }
                }
            }
        }
    }
fn build_response(path: &str) -> Response<Cow<'static, [u8]>> {
    let content = get_response(path.into());

    let mime_type = infer::get(content)
        .map(|file_type| file_type.mime_type())
        .unwrap_or_else(|| match path {
            p if p.ends_with(".html") => "text/html; charset=utf-8",
            p if p.ends_with(".css") => "text/css; charset=utf-8",
            p if p.ends_with(".js") => "application/javascript; charset=utf-8",
            p if p.ends_with(".json") => "application/json; charset=utf-8",
            p if p.ends_with(".svg") => "image/svg+xml",
            p if p.ends_with(".png") => "image/png",
            p if p.ends_with(".ico") => "image/x-icon",
            p if p.ends_with(".wasm") => "application/wasm",
            _ => "text/plain; charset=utf-8",
        });

    Response::builder()
        .header(CONTENT_TYPE, mime_type)
        .body(Cow::Borrowed(content))
        .unwrap_or_else(|_e| Response::default())
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
                .with_devtools(cfg!(debug_assertions))
                // We're not building a browser are we?
                .with_back_forward_navigation_gestures(false)
                // The base URL that asynchronous_custom_protocol would be listening on
                // TODO: this may need tweaks on Android, maybe it should be:
                // http://app.client/assets_bundled/
                .with_url("app://client/assets_bundled/test.txt")
                .with_asynchronous_custom_protocol("app".into(), {
                    move |_id, request, responder| {
                        // We can't move twice, so we clone variables before moving it twice
                        // This has no performance cost for us because we're using the
                        // Automatically Reference Counted (ARC) type, which just
                        // adds refrences to a variable instead of cloning & copying
                        // yet another one.

                        //thread::spawn(move || {
                        let path = request.uri().path().to_string();
                        info!("{path}");

                        // We unwrap the lock of variables here & then take refs

                        // Here we build the actual response
                        responder.respond(
                            build_response(&path)
                        );
                        //});
                    }
                }),
        }
    }
}

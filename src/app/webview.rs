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

pub fn get_asset(path: &str) -> Option<Vec<u8>> {
    let assets = &ASSETS;
    let asset = assets.get(&path);
    match asset {
        Some(result) => Some((*result).into()),
        None => None,
    }
}

#[allow(clippy::match_single_binding)]
pub fn get_response(path: String) -> Vec<u8> {
    match path {
        // Here, we fall back to our assets
        _ => {
            match get_asset(&path) {
                Some(s) => s,
                _ => {
                    error!("Unknown path: {path}");

                    get_asset("/assets_bundled/index.html")
                        // # Safety
                        //
                        // So you're telling me that we have our ASSETS set,
                        // But we don't have a main page?
                        .expect(
                            "webview_manager.rs: Couldn't get the default page, missing assets?",
                        )
                }
            }
        }
    }
}

fn build_response(path: &str) -> Response<Cow<'static, [u8]>> {
    let content = get_response(path.into());
    let bytes = content.as_ref();

    let mime_type = match path.rsplit_once('.') {
        Some((_, "html")) => "text/html; charset=utf-8",
        Some((_, "css")) => "text/css; charset=utf-8",
        Some((_, "js")) => "application/javascript; charset=utf-8",
        Some((_, "json")) => "application/json; charset=utf-8",
        Some((_, "svg")) => "image/svg+xml",
        Some((_, "png")) => "image/png",
        Some((_, "ico")) => "image/x-icon",
        Some((_, "wasm")) => "application/wasm",
        _ => infer::get(bytes)
            .map(|file_type: infer::Type| file_type.mime_type())
            .unwrap_or("text/plain; charset=utf-8"),
    };

    Response::builder()
        .header(CONTENT_TYPE, mime_type)
        .body(Cow::Owned(content))
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
                        responder.respond(build_response(&path));
                        //});
                    }
                }),
        }
    }
}

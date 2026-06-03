use std::collections::HashMap;
use std::sync::OnceLock;
use serde_json::{from_str, from_value};

const ASSETS_JSON: &str =  r#"{"/assets_bundled/test.txt":"test"}"#;

pub static ASSETS: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn init() {
    let assets_map: HashMap<String, String> =
        from_value(
            from_str(ASSETS_JSON)
                .expect("assets_manager.rs: Couldn't parse json/string into var")
        ).expect("assets_manager.rs: Couldn't parse var into map");

    ASSETS.set(assets_map)
        .expect("assets_manager.rs: Couldn't set/lock ASSETS map");
}

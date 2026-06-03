
use serde_json::from_str;
use std::{collections::HashMap, sync::LazyLock};

const ASSETS_JSON: &str =  r#"{"/assets_bundled/test.txt":"test"}"#;

pub static ASSETS: LazyLock<HashMap<String, String>> =
    LazyLock::new(|| from_str(ASSETS_JSON).expect("Couldn't parse json/string into var"));

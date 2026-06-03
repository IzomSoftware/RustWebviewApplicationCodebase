// use std::fs;
// use std::path::Path;
// use std::process::Command;

fn main() {
    // let out = Path::new("target").join("build-rerun-trigger");
    // fs::create_dir_all(out.parent().unwrap()).unwrap();
    // fs::write(
    //     &out,
    //     format!(
    //         "{}",
    //         std::time::SystemTime::now()
    //             .duration_since(std::time::UNIX_EPOCH)
    //             .unwrap()
    //             .as_nanos()
    //     ),
    // )
    // .unwrap();

    // println!("cargo:rerun-if-changed={}", out.display());

    println!("cargo:rerun-if-changed=assets_bundled/");

    // Uncomment to build tailwindcss
    /*execute_command(
        "chmod".to_string(),
        vec!["+x".to_string(), "tailwindcss-linux-x64".to_string()],
    );
    execute_command(
        "./tailwindcss-linux-x64".to_string(),
        vec![
            "-i".to_string(),
            "assets/input.css".to_string(),
            "-o".to_string(),
            "assets/style.css".to_string(),
            "--minify".to_string(),
            "-c".to_string(),
            "tailwind.config.js".to_string(),
        ],
    );*/

    assets::include_asset();
}

// fn execute_command(command: String, args: Vec<String>) {
//     let mut cmd = Command::new(command);

//     for arg in args {
//         cmd.arg(arg);
//     }

//     cmd.spawn().unwrap().wait().unwrap();
// }

mod assets {
    use std::collections::HashMap;
    use std::fs::{read_to_string, write};
    use walkdir::WalkDir;

    fn insert(map: &mut HashMap<String, String>, name: String, content: String) {
        map.insert(name, content);
    }

    pub fn include_asset() {
        let mut temp: HashMap<String, String> = HashMap::new();

        for entry in WalkDir::new("./assets_bundled")
            .into_iter()
            .filter_entry(|p| {
                p.file_name() != "android"
                    && p.file_name() != "windows"
                    && p.file_name() != "linux"
                    && p.file_name() != "universal"
            })
            .filter_map(Result::ok)
        {
            if entry.file_type().is_file() {
                let path = entry.path();
                let rel = path.strip_prefix("./assets_bundled").unwrap_or(path);
                let key = format!(
                    "/assets_bundled/{}",
                    rel.components()
                        .map(|c| c.as_os_str().to_string_lossy())
                        .collect::<Vec<_>>()
                        .join("/")
                );
                insert(
                    &mut temp,
                    key.parse().unwrap(),
                    read_to_string(path).unwrap(),
                );
            }
        }

        for s in &temp {
            println!("cargo:warning=including file {} in the binary", &s.0);
        }

        let json = serde_json::to_string(&temp).unwrap();

        let code = r###"
use serde_json::from_str;
use std::{collections::HashMap, sync::LazyLock};

const ASSETS_JSON: &str =  r#""#;

pub static ASSETS: LazyLock<HashMap<String, String>> =
    LazyLock::new(|| from_str(ASSETS_JSON).expect("Couldn't parse json/string into var"));
"###;

        let code = code.replace(
            "const ASSETS_JSON: &str =  r#\"\"#;",
            &format!("const ASSETS_JSON: &str =  r#\"{}\"#;", json.as_str()),
        );

        write("target/assets_bundled_manager.rs", code).unwrap();
    }
}

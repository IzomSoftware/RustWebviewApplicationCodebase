// use std::fs;
// use std::path::Path;
// use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=assets_bundled/");

    // Uncomment to build tailwindcss before including assets
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
    use std::collections::BTreeMap;
    use std::fs::{write};
    use walkdir::WalkDir;

    pub fn include_asset() {
        let mut temp = BTreeMap::new();

        let entries = WalkDir::new("./assets_bundled")
            .into_iter()
            .filter_entry(|p| {
                p.file_name() != "android"
                    && p.file_name() != "windows"
                    && p.file_name() != "linux"
                    && p.file_name() != "universal"
            })
            .filter_map(Result::ok);

        for entry in entries
        {
            if entry.file_type().is_file() {
                let path = entry.path();
                let rel = path.strip_prefix("./").unwrap_or(path);
                temp.insert(format!("/{}", rel.to_string_lossy()), rel.to_string_lossy().to_string());
            }
        }

        if temp.is_empty() {
            println!("cargo:warning=Empty assets_bundled path");
            return;
        }

        let mut temp_ent = Vec::new();
        for s in &temp {
            temp_ent.push(format!(
                "    \"{}\" => include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{}\")),",
                s.0, s.1));
            println!("cargo:warning=including file {} in the binary", &s.0);
        }

       let code = format!(
r#"use phf::phf_map;

pub static ASSETS: phf::Map<&'static str, &'static [u8]> = phf_map! {{
{}
}};"#,
            temp_ent.join("\n")
        );

        write("target/bundled.rs", code).unwrap();
    }
}

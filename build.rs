use ron;
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    println!("Creating asset manifest");
    let assets_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets");
    let mut assets: HashMap<String, Vec<String>> = [
        ("campaigns".into(), vec![]),
        ("fonts".into(), vec![]),
        ("maps".into(), vec![]),
        ("patterns".into(), vec![]),
        ("saves".into(), vec![]),
        ("sprites".into(), vec![]),
        ("themes".into(), vec![]),
    ]
    .iter()
    .cloned()
    .collect();

    assets.iter_mut().for_each(|(folder, files)| {
        // get every asset directory and map the contents
        assets_dir
            .join(folder)
            .read_dir()
            .unwrap()
            .for_each(|asset| {
                // map to iterator and then map to success to get the inner item
                let asset = asset.unwrap();
                files.push(asset.path().file_name().unwrap().to_str().unwrap().into());
            });
    });

    let assets = Container(assets);

    let mut f = File::create(assets_dir.join("assets.manifest")).unwrap();

    if let Ok(text) = ron::to_string(&assets) {
        let mut file = File::create(assets_dir.join("assets.manifest")).unwrap();
        file.write_all(text.as_bytes()).ok();
    }
}

/// Container to match format on game side
#[derive(Serialize)]
struct Container(pub HashMap<String, Vec<String>>);

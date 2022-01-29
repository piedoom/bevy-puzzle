use serde::{self, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{DirEntry, File};
use std::io::Write;
use std::path::Path;

// Path to the manifest that will be created (relative to Cargo.toml)
const PATH: &str = "assets.manifest";

fn main() {
    let assets_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets");

    // Here we list all the folders that assets are loaded within. It's
    // important to note that this implementation does not account for nested
    // directories. If you are looking for that, maybe try the `walk` crate. In
    // my case, I have a whole ton of directories (campaigns, maps, etc.)
    let mut assets: HashMap<String, Vec<String>> = [
        ("campaigns".into(), vec![]),
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
        let mut assets: Vec<DirEntry> = assets_dir
            .join(folder)
            .read_dir()
            .unwrap()
            .map(|asset| {
                // map to iterator and then map to success to get the inner item
                asset.unwrap()
            })
            .collect();
        for asset in assets.iter_mut() {
            files.push(asset.path().file_name().unwrap().to_str().unwrap().into());
        }
        files.sort();
    });

    // Mutate the hashmap in place with contents from the asset directory
    let mut list = assets
        .iter()
        .map(|x| (x.0.clone(), x.1.clone()))
        .collect::<Vec<(String, Vec<String>)>>();
    list.sort();

    // Put the assets within a struct just so it saves in RON a bit nicer within
    // a struct. You can also probably just do some funky `format!()` stuff here
    let assets = Container(assets);
    File::create(assets_dir.join(PATH)).unwrap();

    // Save the resulting RON manifest in the assets folder. Depending on what
    // you want you might want to put this elsewhere that isn't committed to
    // VCS, like target
    if let Ok(text) = ron::to_string(&assets) {
        let mut file = File::create(assets_dir.join(PATH)).unwrap();
        file.write_all(text.as_bytes()).ok();
    }
}

/// Container to match format on game side
#[derive(Serialize)]
struct Container(pub HashMap<String, Vec<String>>);

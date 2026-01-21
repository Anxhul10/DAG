// Use #[neon::export] to export Rust functions as JavaScript functions.
// See more at: https://docs.rs/neon/latest/neon/attr.export.html
mod find_pkg_json;
mod utils;

use serde_json::Value;

fn get_pkg_name(path: String) -> String{
    let payload: Value = 
     utils::read_payload::read_payload(path).unwrap();
    let name = payload.get("name")
        .expect("file should have FirstName key").to_string();
    name
}

fn get_tree_deps(path: String) -> String {    
    let payload: Value = match utils::read_payload::read_payload(path) {
        Ok(p) => p,
        Err(_) => return String::new(),
    };

    match payload.get("dependencies") {
        Some(v) => v.to_string(),
        None => String::new(),
    }
}

fn get_tree_dev(path: String, current_pkg: String) -> String {
    let payload: Value = match utils::read_payload::read_payload(path) {
        Ok(p) => p,
        Err(_) => return String::new(),
    };

    match payload.get("devDependencies") {
        Some(v) => v.to_string(),
        None => String::new(),
    }
}

fn get_tree_peer(path: String) -> String {    
    let payload: Value = match utils::read_payload::read_payload(path) {
        Ok(p) => p,
        Err(_) => return String::new(),
    };

    match payload.get("peerDependencies") {
        Some(v) => v.to_string(),
        None => String::new(),
    }
}
#[neon::export]
fn runner(name: String) -> String {
    let mut pkg_names = Vec::new();
    let filter = vec![ ".yarn", "node_modules"];
    let paths = find_pkg_json::find_pkg_json(filter);
    for path in paths.clone() {
        pkg_names.push(get_pkg_name(path.clone()));
    }

    let mut pkg_tracker = 0;
    for path in paths.clone() {
        let current_pkg = pkg_names[pkg_tracker].clone();
        let result = get_tree_dev(path.clone(), current_pkg).clone();
        println!("{}", result);
        pkg_tracker += 1;
    }
    format!("hello {name}")
}

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

fn get_dependents(path: &str, pkg_names: &[String]) {
    let payload: Value = utils::read_payload::read_payload(path).unwrap();

    let name_value = payload.get("name");

    let pkg_json_name = name_value
        .and_then(|v| v.as_str())
        .expect("payload should have `name` key");

    let sections = ["dependencies", "peerDependencies", "devDependencies"];

    for section in &sections {
        let section_value = payload.get(section);

        if let Some(value) = section_value {
            let as_object = value.as_object();
            if let Some(obj) = as_object {
                for (dep, _version) in obj {
                    for pkg in pkg_names {
                        println!("{}", pkg.as_str());
                        println!("{}", dep.to_string());
                    }
                }
            }
        }
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

    for path in paths.clone() {
        get_dependents(&path, &pkg_names);
        
    }
    format!("hello {name}")
}

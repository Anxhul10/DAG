mod find_pkg_json;
mod utils;
use neon::prelude::*;
use std::collections::HashSet;
use serde_json::Value;

fn get_pkg_name(path: String) -> String {
    let payload: Value =
        utils::read_payload::read_payload(path).unwrap();

    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .expect("payload should have `name` key");

    name.to_owned()
}

fn get_dependents(path: &str, pkg_names: &[String]) -> Vec<(String, String)>{
    let mut graph: Vec<(String, String)> = Vec::new();
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
                      // checks if the dependency exists on the package.json
                        if pkg == dep {
                            // dependents : dependency
                            graph.push((pkg_json_name.to_string(), dep.to_string()));
                        }
                    }
                }
            }
        }
    }
    graph
}

fn recursion(pkg_name: String, res: Vec<(String, String)>, addon:&mut Vec<String>) {
    for (_i, (dependent, dependency)) in res.clone().into_iter().enumerate() {
        if dependency == pkg_name {
            addon.push(dependent.clone());
            recursion(dependent.clone(), res.clone(),addon);
        }
    }
    return;
}
#[neon::export]
fn get_affected_pkg(pkg_name: String) {
    let filter = vec![".yarn", "node_modules"];
    let paths = find_pkg_json::find_pkg_json(filter);
    let mut res: Vec<(String, String)> = Vec::new();
    let mut pkg_names = Vec::new();
    let mut hash_store = HashSet::<String>::new();

    for path in paths.clone() {
        pkg_names.push(get_pkg_name(path.clone()));
    }

    for path in paths.clone() {
        let mut r = get_dependents(&path, &pkg_names);
        res.append(&mut r);
    }
    let mut addon = Vec::new();
    recursion(pkg_name, res,&mut addon);
    if addon.len() == 0  {
        println!("No dependents found on this pkg");
    }
    else {
        for pkg in addon {
            hash_store.insert(pkg);
        }
        for pkg in hash_store {
            println!("{}", pkg);
        }

    }
}
#[neon::export]
fn dag<'a>(cx: &mut FunctionContext<'a>) -> JsResult<'a, JsArray> {
    let js_array = JsArray::new(cx, 0);

    let mut pkg_names = Vec::new();
    let mut res: Vec<(String, String)> = Vec::new();

    let filter = vec![".yarn", "node_modules"];
    let paths = find_pkg_json::find_pkg_json(filter);

    for path in paths.clone() {
        pkg_names.push(get_pkg_name(path.clone()));
    }

    for path in paths.clone() {
        let mut r = get_dependents(&path, &pkg_names);
        res.append(&mut r);
    }

    for (i, (dependent, dependency)) in res.into_iter().enumerate() {
        let entry = cx.empty_object();

        let js_dep = cx.string(dependency);
        entry.set(cx, dependent.as_str(), js_dep)?; 

        js_array.set(cx, i as u32, entry)?;
    }
    Ok(js_array)
}

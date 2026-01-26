mod find_pkg_json;
mod utils;
use neon::prelude::*;

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

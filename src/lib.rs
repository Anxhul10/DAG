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

#[neon::export]
fn runner(name: String) -> String {
    let filter = vec![ ".yarn", "node_modules"];
    let val = find_pkg_json::find_pkg_json(filter);
    for x in val {
        println!("{}",get_pkg_name(x));
    }
    format!("hello {name}")
}

// Use #[neon::main] to add additional behavior at module loading time.
// See more at: https://docs.rs/neon/latest/neon/attr.main.html
// fn temp() -> Result<()> {
//     for entry in glob("**/package.json")? {
//         println!("{}", entry?.display());
//     }
//     Ok(())
// }

// #[neon::main]
// fn main(mut cx: ModuleContext) -> NeonResult<()> {
//     println!("module is loaded!");
//     Ok(())
// }

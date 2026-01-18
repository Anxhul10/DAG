// Use #[neon::export] to export Rust functions as JavaScript functions.
// See more at: https://docs.rs/neon/latest/neon/attr.export.html
mod find_pkg_json;

#[neon::export]
fn runner(name: String) -> String {
    let filter = vec![ ".yarn", "node_modules"];
    let val = find_pkg_json::find_pkg_json(filter);

    for x in val {
        println!("{}", x.display());
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

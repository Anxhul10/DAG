use glob::glob;
use anyhow::Result;

pub fn find_pkg_json() -> Result<()> {
    for entry in glob("**/package.json")? {
        println!("{}", entry?.display());
    }
    Ok(())
}
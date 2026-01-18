use glob::glob;
use std::path::PathBuf;

pub fn find_pkg_json(filters: Vec<&str>) -> Vec<PathBuf> {
    let mut pkg_paths: Vec<PathBuf> = Vec::new();
    for entry in glob("**/package.json").expect("failed to read glob patterns") {
        for to_filter in &filters {
            match entry {
                Ok(ref path) => {
                    if path.display().to_string().contains(to_filter) == false {
                        pkg_paths.push(path.to_path_buf());
                        break;
                    }
                },
                Err(ref e) => println!("{:?}", e)
            }
        }
    }
    pkg_paths
}
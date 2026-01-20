use glob::glob;

pub fn find_pkg_json(filters: Vec<&str>) -> Vec<String> {
    let mut pkg_paths: Vec<String> = Vec::new();
    for entry in glob("**/package.json").expect("failed to read glob patterns") {
        let mut flag = true;
        for to_filter in &filters {
            match entry {
                Ok(ref path) => {
                    if path.display().to_string().contains(to_filter) {
                        flag = false;
                    }
                },
                Err(ref e) => println!("{:?}", e)
            }
        }
        if flag {
            match entry {
                Ok(path) => {
                    pkg_paths.push(path.display().to_string());
                },
                Err( e) => println!("{:?}", e)
            }
        }
    }
    pkg_paths
}
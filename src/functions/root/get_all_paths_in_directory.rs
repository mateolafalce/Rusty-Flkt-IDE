#[allow(dead_code)]
pub fn get_all_paths_in_directory(
    path: &std::path::Path,
    prefix: String,
    is_the_repository_clear: bool
) -> Vec<String> {
    // If the repository is clear, return an empty path
    if is_the_repository_clear {
        let clear_path: Vec<String> = vec!["".to_string()];
        clear_path
    } else {
        let mut paths = Vec::new();
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let mut path_str = path.to_str().unwrap().to_string().replace("\\", "/");
            if path.is_dir() {
                path_str.push('/');
                // Recursively get sub-paths
                let sub_paths = get_all_paths_in_directory(&path, prefix.clone(), is_the_repository_clear);
                paths.extend(sub_paths);
            }
            paths.push(path_str);
        }
        // Remove the prefix from the paths
        for i in 0..paths.len() {
            if paths[i].starts_with(&prefix) {
                paths[i] = paths[i].replace(&prefix, "");
            }
        }
        paths
    }
}

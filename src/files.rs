use std::path::PathBuf;

thread_local!(static BASE_DIRS: xdg::BaseDirectories = xdg::BaseDirectories::with_prefix("rustfolio").expect("Failed to create data directories"));

pub fn get_full_path<T: AsRef<std::path::Path>>(file_name: T) -> Result<PathBuf, std::io::Error> {
    BASE_DIRS.with(|dir| {
        dir.place_data_file(file_name)
    })
}

pub fn list_data_files() ->  Vec<PathBuf> {
    BASE_DIRS.with(|dir| {
        dir.list_data_files(std::path::Path::new(""))
    })
}

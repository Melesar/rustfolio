use std::path::PathBuf;

thread_local!(static BASE_DIRS: xdg::BaseDirectories = xdg::BaseDirectories::with_prefix("rustfolio").expect("Failed to create data directories"));

pub fn get_full_path(file_name: String) -> Result<PathBuf, std::io::Error> {
    BASE_DIRS.with(|dir| {
        dir.place_data_file(file_name)
    })
}

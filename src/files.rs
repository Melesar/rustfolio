use std::path::PathBuf;


pub fn get_full_path(file_name: String) -> Result<PathBuf, std::io::Error> {
    let base_dirs = xdg::BaseDirectories::new()?;
    base_dirs.place_data_file(file_name)
}

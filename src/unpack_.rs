fn create(file: &marko_fs_types::File, where_:&std::path::PathBuf) -> Result<(), std::io::Error> {
    use std::io::Write; write!(std::fs::File::create(where_.join(&file.name))?, "{}", file.contents)?;
Ok(())}

pub fn unpack(packedFsFolder: &marko_fs_types::Folder, where_:&std::path::PathBuf) -> Result<(), std::io::Error> {
    let fp = where_.join(&packedFsFolder.name);
    std::fs::create_dir_all(&fp)?;
    for subfolder in &packedFsFolder.subfolders { unpack(&subfolder, &fp)?; }
    for file in &packedFsFolder.files { create(&file, &fp)?; }
Ok(())}

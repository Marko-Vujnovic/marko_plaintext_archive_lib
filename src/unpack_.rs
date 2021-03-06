fn create(file: &marko_fs_types::File, where_:&std::path::PathBuf) -> core::result::Result<(), std::io::Error> {
    use std::io::Write; write!(std::fs::File::create(where_.join(&file.name))?, "{}", file.contents)?;
Ok(())}

pub fn unpack(packed_fs_folder: &marko_fs_types::Folder, where_:&std::path::Path) -> core::result::Result<(), std::io::Error> {
    let fp = where_.join(&packed_fs_folder.name);
    std::fs::create_dir_all(&fp)?;
    for subfolder in &packed_fs_folder.subfolders { unpack(&subfolder, &fp)?; }
    for file in &packed_fs_folder.files { create(&file, &fp)?; }
Ok(())}

pub fn unpack2<P: AsRef<std::path::Path>, P2: AsRef<std::path::Path>>(mpa_file: P, where_: P2) -> core::result::Result<(), std::io::Error> {
    let folder: marko_fs_types::Folder = serde_yaml::from_str(&std::fs::read_to_string(&mpa_file).expect("Couldn't read the file")).unwrap();
    unpack(&folder, where_.as_ref())
}
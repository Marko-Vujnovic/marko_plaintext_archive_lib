use crate as marko_plaintext_archive; use marko_plaintext_archive::*;

pub fn pack(folder_to_pack:&std::path::Path) -> core::result::Result<marko_fs_types::Folder, std::io::Error> {
    let mut root_folder = marko_fs_types::Folder{name:to_String(&folder_to_pack), subfolders: vec![], files: vec![]};

    for dir_entry in std::fs::read_dir(folder_to_pack).unwrap() { let dir_entry = dir_entry?; let path = dir_entry.path();
        if path.is_dir() {
            let subfolder = pack(&path)?;
            root_folder.subfolders.push(subfolder);
        }
        else {
            let file = marko_fs_types::File{name:osstrref_to_String(path.file_name().unwrap()), contents:std::fs::read_to_string(path)?};
            root_folder.files.push(file);
        }
    }
Ok(root_folder)}

pub fn pack2<P: AsRef<std::path::Path>, P2: AsRef<std::path::Path>>(folder_to_pack: P, where_: P2) -> core::result::Result<(), std::io::Error> {
    let in_ram_fs_tree = pack(folder_to_pack.as_ref())?;
    use std::io::Write; write!(std::fs::File::create(format!("{}.mpa", to_string_(&where_.as_ref().join(folder_to_pack.as_ref().file_name().unwrap()))))?, "{}", yaml_beautify(&serde_yaml::to_string(&in_ram_fs_tree).unwrap()))?;
Ok(())}
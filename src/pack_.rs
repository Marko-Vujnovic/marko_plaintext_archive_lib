// use super::to_String::{to_String, osstrref_to_String};
use super::to_String::*;

pub fn pack(folder_to_pack:&std::path::PathBuf) -> Result<marko_fs_types::Folder, std::io::Error> {
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

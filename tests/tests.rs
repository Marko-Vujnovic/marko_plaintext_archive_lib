// #![feature(plugin)]
// #![plugin(serde_macros)]

//! Copyright © Marko Vujnovic, GPLv3 license

// use crate as marko_plaintext_archive;
use marko_plaintext_archive::*;

#[test]
fn main_t() -> core::result::Result<(), std::io::Error> { tokio::runtime::Runtime::new().unwrap().block_on(async {

    marko_plaintext_archive::entry_point(&["mpa", "unpack", "tests/ExampleProject.mpa", "produced"])?;
    marko_plaintext_archive::entry_point(&["mpa", "pack", "tests/ExampleProject", "produced"])?;

    Ok(())})}

#[test]
fn roundtrip_t() -> core::result::Result<(), std::io::Error> { tokio::runtime::Runtime::new().unwrap().block_on(async {
    std::fs::create_dir_all(&get_app_folder())?;

    let serialized_file = cwd().join(format!("tests/{}.mpa", "ExampleProject")); // .mpa - "Marko plain-text archive"

    let orig_folder = cwd().join("tests/ExampleProject");
    let in_ram_fs_tree = pack(&orig_folder)?;

    use std::io::Write; write!(std::fs::File::create(cwd().join(format!("tests/{}.mpa", "ExampleProject")))?, "{}", yaml_beautify(&serde_yaml::to_string(&in_ram_fs_tree).unwrap()))?;

    let produced_folder = cwd().join(format!("produced/{}", "ExampleProject"));
    let produced_folders_parent = produced_folder.parent().unwrap().to_path_buf();

    unpack(&in_ram_fs_tree, &produced_folders_parent);
    assert!(folders_are_the_same(&orig_folder, &produced_folder)?);

    unpack2(&serialized_file, &produced_folders_parent);
    assert!(folders_are_the_same(&orig_folder, &produced_folder)?);

    unpack2("tests/ExampleProject.mpa", &produced_folders_parent);
    assert!(folders_are_the_same(&orig_folder, &produced_folder)?);

    let script_project: marko_fs_types::Folder = serde_yaml::from_str(&std::fs::read_to_string(&serialized_file).expect("Something went wrong reading the file")).unwrap();

    let serialized_str = serde_yaml::to_string(&script_project).unwrap();
    write!(std::fs::File::create(cwd().join(format!("produced/{}.mpa", "ExampleProject")))?, "{}", yaml_beautify(&serialized_str))?;
Ok(())})}


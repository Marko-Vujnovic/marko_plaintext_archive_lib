// #![feature(plugin)]
// #![plugin(serde_macros)]

//! Copyright © Marko Vujnovic, GPLv3 license

// static programName: &'static str = "mpa";
struct ProgramInfo { name: &'static str }
static PROGRAM_INFO: ProgramInfo = ProgramInfo {
    name: "mpa",
};

fn get_app_folder() -> std::path::PathBuf { let mut dir = std::path::PathBuf::new(); dir.push(&std::env::var("HOME").unwrap()); dir.join(&PROGRAM_INFO.name) }
fn cwd() -> std::path::PathBuf { std::env::current_dir().unwrap() }

use marko_plaintext_archive::*;

fn mpa_str(folder: &std::path::PathBuf) -> core::result::Result<String, std::io::Error> { Ok(serde_yaml::to_string(&pack(&folder)?).unwrap()) }
fn folders_are_the_same(left: &std::path::PathBuf, right: &std::path::PathBuf) -> core::result::Result<bool, std::io::Error> { Ok(mpa_str(&left)?.eq(&mpa_str(&right)?)) }


fn yaml_beautify(serialized_str: &str) -> std::string::String {
    let parsed = yaml_rust::YamlLoader::load_from_str(&serialized_str).unwrap();
    let mut output = String::new();
    let mut emitter = yaml_rust::YamlEmitter::new(&mut output);
    emitter.multiline_strings(true);
    emitter.dump(&parsed[0]).unwrap();
    output
}

#[test]
fn roundtrip_t() -> core::result::Result<(), std::io::Error> { tokio::runtime::Runtime::new().unwrap().block_on(async {
    std::fs::create_dir_all(&get_app_folder())?;

    let orig_folder = cwd().join("tests/ExampleProject");
    let in_ram_fs_tree = pack(&orig_folder)?;

    use std::io::Write; write!(std::fs::File::create(cwd().join(format!("tests/{}.mpa", "ExampleProject")))?, "{}", yaml_beautify(&serde_yaml::to_string(&in_ram_fs_tree).unwrap()))?;

    let produced_folder = cwd().join(format!("produced/{}", "ExampleProject"));
    unpack(&in_ram_fs_tree, &produced_folder.parent().unwrap().to_path_buf());

    assert!(folders_are_the_same(&orig_folder, &produced_folder)?);

    let serialized_file = cwd().join(format!("tests/{}.mpa", "ExampleProject")); // .mpa - "Marko plain-text archive"
    let script_project: marko_fs_types::Folder = serde_yaml::from_str(&std::fs::read_to_string(&serialized_file).expect("Something went wrong reading the file")).unwrap();

    let serialized_str = serde_yaml::to_string(&script_project).unwrap();
    write!(std::fs::File::create(cwd().join(format!("produced/{}.mpa", "ExampleProject")))?, "{}", yaml_beautify(&serialized_str))?;


Ok(())})}


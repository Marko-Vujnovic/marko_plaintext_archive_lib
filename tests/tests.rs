//! Copyright © Marko Vujnovic, GPLv3 license

// static programName: &'static str = "mpa";
struct ProgramInfo { name: &'static str }
static program_info: ProgramInfo = ProgramInfo {
    name: "mpa",
};

fn get_app_folder() -> std::path::PathBuf { let mut dir = std::path::PathBuf::new(); dir.push(&std::env::var("HOME").unwrap()); dir.join(&program_info.name) }
fn cwd() -> std::path::PathBuf { std::env::current_dir().unwrap() }

use marko_plaintext_archive::*;

fn mpa_str(folder: &std::path::PathBuf) -> Result<String, std::io::Error> { Ok(serde_json::to_string(&pack(&folder)?)?) }
fn folders_are_the_same(left: &std::path::PathBuf, right: &std::path::PathBuf) -> Result<bool, std::io::Error> { Ok(mpa_str(&left)?.eq(&mpa_str(&right)?)) }


#[test]
fn roundtripT() -> Result<(), std::io::Error> { tokio::runtime::Runtime::new().unwrap().block_on(async {
    std::fs::create_dir_all(&get_app_folder())?;

    let orig_folder = cwd().join("tests/ExampleProject");
    let in_ram_fs_tree = pack(&orig_folder)?;

    use std::io::Write; write!(std::fs::File::create(cwd().join(format!("tests/{}.mpa", "ExampleProject")))?, "{}", serde_json::to_string(&in_ram_fs_tree)?)?;

    let produced_folder = cwd().join(format!("produced/{}", "ExampleProject"));
    unpack(&in_ram_fs_tree, &produced_folder.parent().unwrap().to_path_buf());

    assert!(folders_are_the_same(&orig_folder, &produced_folder)?);

    let jsonFile = cwd().join(format!("tests/{}.mpa", "ExampleProject")); // .mpa - "Marko plain-text archive"
    let scriptProject: marko_fs_types::Folder = serde_json::from_str(&std::fs::read_to_string(&jsonFile).expect("Something went wrong reading the file"))?;

    writeln!(std::fs::File::create(cwd().join(format!("produced/{}.mpa", "ExampleProject")))?, "{}", serde_json::to_string(&scriptProject)?)?;

Ok(())})}


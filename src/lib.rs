pub mod pack_; pub use pack_::*;
pub mod unpack_; pub use unpack_::*;
pub mod to_String; pub use to_String::*;

// use crate as marko_plaintext_archive; use marko_plaintext_archive::*;

pub fn yaml_beautify(serialized_str: &str) -> std::string::String {
    let parsed = yaml_rust::YamlLoader::load_from_str(&serialized_str).unwrap();
    let mut output = String::new();
    let mut emitter = yaml_rust::YamlEmitter::new(&mut output);
    emitter.multiline_strings(true);
    emitter.dump(&parsed[0]).unwrap();
    output
}

pub fn to_string_(p: &std::path::Path) -> String {
    p.to_str().unwrap().to_string()
}

struct ProgramInfo { name: &'static str }
static PROGRAM_INFO: ProgramInfo = ProgramInfo {
    name: "mpa",
};

pub fn get_app_folder() -> std::path::PathBuf { let mut dir = std::path::PathBuf::new(); dir.push(&std::env::var("HOME").unwrap()); dir.join(&PROGRAM_INFO.name) }
pub fn cwd() -> std::path::PathBuf { std::env::current_dir().unwrap() }

use crate::Operation::Pack;

fn mpa_str(folder: &std::path::PathBuf) -> core::result::Result<String, std::io::Error> { Ok(serde_yaml::to_string(&pack(&folder)?).unwrap()) }
pub fn folders_are_the_same(left: &std::path::PathBuf, right: &std::path::PathBuf) -> core::result::Result<bool, std::io::Error> { Ok(mpa_str(&left)?.eq(&mpa_str(&right)?)) }

pub enum Operation { Pack, Unpack }

#[derive(clap::Parser, Debug)]
struct CliArgs {
    #[clap(subcommand)]
    pub operation: Ops,
}

pub trait Applicable { fn apply(&self) -> (); }

#[derive(clap::Parser, Debug, Default)]
// struct PackArgs { #[clap(short, long)] pub folder: Option<String>, }
struct PackArgs { pub folder: String, pub where_: String }

#[derive(clap::Parser, Debug, Default)]
struct UnpackArgs { pub file: String, pub where_: String }

impl Applicable for PackArgs { fn apply(&self) -> () {
    pack2(&self.folder, &self.where_);
}}

impl Applicable for UnpackArgs { fn apply(&self) -> () {
    unpack2(&self.file, &self.where_);
}}

#[derive(clap::Parser, Debug, strum::EnumString, strum::Display, strum::AsRefStr)]
enum Ops {
    // Pack { folder: Option<String>, },
    Pack(PackArgs),
    Unpack(UnpackArgs),
    Version,
}

impl Applicable for Ops { fn apply(&self) -> () {
    match self {
        Ops::Unpack(args) => { args.apply(); }
        Ops::Pack(args) => { args.apply(); }
        _ => {}
    }
}}

pub fn entry_point(args_: &[&str]) -> core::result::Result<(), std::io::Error> {
    use clap::Parser; let args = if args_.len() == 0 { CliArgs::parse() } else { CliArgs::from_iter(args_) };
    args.operation.apply();
    Ok(())
}

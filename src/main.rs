use pest_meta::parse_and_optimize;
use pest_vm::Vm;
use serde::Deserialize;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
};
use toml::{from_str, Table};

struct Workspace {
    root: PathBuf,
}

impl Workspace {
    fn get_path<P: AsRef<Path>>(&self, relative: P) -> PathBuf {
        PathBuf::from_iter([self.root.as_ref(), relative.as_ref()])
    }

    fn get_meta<P: AsRef<Path>>(&self, relative: P) -> PathBuf {
        PathBuf::from_iter([self.root.as_ref(), Path::new(".remedy"), relative.as_ref()])
    }

    fn get_config(&self) -> Configuration {
        let mut config = File::open(self.get_meta("config.toml")).expect("Error opening config");
        let mut buffer = String::new();
        config.read_to_string(&mut buffer).unwrap();
        from_str::<Configuration>(&buffer).expect("Error parsing config")
    }
}

#[derive(Deserialize, Debug)]
struct Configuration {
    profile: Vec<Profile>,
}

#[derive(Deserialize, Debug)]
struct Profile {
    grammar: PathBuf,
    rule: String,
}

fn main() {
    let workspace = Workspace {
        root: PathBuf::from_str("/home/rowan/wkspc").unwrap(),
    };
    let config = workspace.get_config();
    let profile = &config.profile[0];
    let mut grammar =
        File::open(workspace.get_meta(&profile.grammar)).expect("Error opening grammar");
    let mut buffer = String::new();
    grammar.read_to_string(&mut buffer).unwrap();
    let (_, rules) = parse_and_optimize(&buffer).expect("Error parsing grammar");
    let vm = Vm::new(rules);

    let mut file = File::open(workspace.get_path("test.rem")).expect("Error opening file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let pairs = vm
        .parse(&profile.rule, &buffer)
        .expect("Error parsing file with grammar");
    println!("{:#?}", pairs);
}

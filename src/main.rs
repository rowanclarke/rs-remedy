use serde::Deserialize;
use std::{fs::File, io::Read, path::PathBuf, str::FromStr};
use toml::{from_str, Table};

struct Workspace {
    root: PathBuf,
}

impl Workspace {
    fn get_config(&self) -> Configuration {
        let mut config = self.root.clone();
        config.push(".remedy");
        config.push("config.toml");
        let mut config = File::open(config).expect("Error opening config");
        let mut buffer = String::new();
        config.read_to_string(&mut buffer).unwrap();
        from_str::<Configuration>(&buffer).expect("Error parsing configuration")
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
    println!("{:?}", workspace.get_config());
}

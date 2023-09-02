use anyhow::Result;
use serde::Deserialize;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use toml::from_str;

pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn get_path<P: AsRef<Path>>(&self, relative: P) -> PathBuf {
        PathBuf::from_iter([self.root.as_ref(), relative.as_ref()])
    }

    pub fn get_meta<P: AsRef<Path>>(&self, relative: P) -> PathBuf {
        PathBuf::from_iter([self.root.as_ref(), Path::new(".remedy"), relative.as_ref()])
    }

    pub fn get_meta_deserialized<T: for<'de> Deserialize<'de>, P: AsRef<Path>>(
        &self,
        relative: P,
    ) -> Result<T> {
        let mut config = File::open(self.get_meta(relative))?;
        let mut buffer = String::new();
        config.read_to_string(&mut buffer).unwrap();
        Ok(from_str::<T>(&buffer)?)
    }
}

use ron::de::from_reader;
use serde::de::DeserializeOwned;
use std::{fs::File, io::BufReader, path::Path};

pub fn load_ron<P, T>(path: P) -> anyhow::Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    Ok(from_reader(BufReader::new(File::open(path)?))?)
}

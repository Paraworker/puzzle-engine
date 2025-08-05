use crate::RulesError;
use ron::de::from_reader;
use serde::de::DeserializeOwned;
use std::{fs::File, io::BufReader, path::Path};

pub(crate) fn load_ron<P, T>(path: P) -> Result<T, RulesError>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    Ok(from_reader(BufReader::new(File::open(path)?))?)
}

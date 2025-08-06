use crate::RulesError;
use ron::{de::from_reader, from_str, ser::{to_string_pretty, PrettyConfig}};
use serde::{de::DeserializeOwned, Serialize};
use std::{fs::{self, File}, io::BufReader, path::Path};

pub(crate) fn ron_from_str<P, T>(str: &str) -> Result<T, RulesError>
where
    T: DeserializeOwned,
{
    Ok(from_str(str)?)
}

pub(crate) fn ron_to_str<T>(value: &T) -> Result<String, RulesError>
where
    T: Serialize,
{
    Ok(to_string_pretty(value, PrettyConfig::default())?)
}

pub(crate) fn ron_from_file<P, T>(path: P) -> Result<T, RulesError>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    Ok(from_reader(BufReader::new(File::open(path)?))?)
}

pub(crate) fn ron_to_file<T, P>(value: &T, path: P) -> Result<(), RulesError>
where
    T: Serialize,
    P: AsRef<Path>,
{
    Ok(fs::write(path, ron_to_str(value)?)?)
}

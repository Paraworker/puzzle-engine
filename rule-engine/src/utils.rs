use crate::RulesError;
use ron::{
    de::from_reader,
    from_str,
    ser::{PrettyConfig, to_string_pretty},
};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

pub(crate) fn from_ron_str<T>(str: &str) -> Result<T, RulesError>
where
    T: DeserializeOwned,
{
    Ok(from_str(str)?)
}

pub(crate) fn to_ron_str<T>(value: &T) -> Result<String, RulesError>
where
    T: Serialize,
{
    let pretty = PrettyConfig::new().separate_tuple_members(true);
    Ok(to_string_pretty(value, pretty)?)
}

pub(crate) fn from_ron_file<P, T>(path: P) -> Result<T, RulesError>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    Ok(from_reader(BufReader::new(File::open(path)?))?)
}

pub(crate) fn to_ron_file<T, P>(value: &T, path: P) -> Result<(), RulesError>
where
    T: Serialize,
    P: AsRef<Path>,
{
    Ok(fs::write(path, to_ron_str(value)?)?)
}

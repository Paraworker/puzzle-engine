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

pub const fn half_width(unit_num: usize, unit_size: f32) -> f32 {
    (unit_num as f32 - 1.0) * unit_size / 2.0
}

use csv::{
    Reader,
    ReaderBuilder,
};
use std::{
    error::Error,
    fs::File,
    path::PathBuf,
};

pub trait PathReader<T>
{
    fn csv(&self) -> Result<Reader<File>, Box<dyn Error>>;
}

impl PathReader<File> for PathBuf
{
    fn csv(&self) -> Result<Reader<File>, Box<dyn Error>>
    {
        ReaderBuilder::new().from_path(self).map_err(Into::into)
    }
}

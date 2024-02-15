use super::common::*;
use log::trace;
use std::{
    error::Error,
    path::PathBuf,
};

pub trait RowPrinter
{
    fn print_all_rows(&self) -> Result<(), Box<dyn Error>>;
}

impl RowPrinter for PathBuf
{
    fn print_all_rows(&self) -> Result<(), Box<dyn Error>>
    {
        for result in self.csv()?.records() {
            let record = result?;
            trace!("{:?}", record);
        }

        Ok(())
    }
}

use crate::csv_processor::error::*;
use csv::{
    Reader,
    ReaderBuilder,
};
use log::{
    debug,
    trace,
};
use std::{
    error::Error,
    fs::File,
    path::PathBuf,
};

pub struct CsvProcessor
{
    file_path: PathBuf,
}

impl CsvProcessor
{
    pub fn new(file_path: PathBuf) -> Result<CsvProcessor, Box<dyn Error>>
    {
        Ok(CsvProcessor { file_path })
    }

    pub fn reader(&self) -> Result<Reader<File>, Box<dyn Error>>
    {
        ReaderBuilder::new()
            .has_headers(true)
            .from_path(&self.file_path)
            .map_err(Into::into)
    }

    pub fn print_all_rows(&mut self) -> Result<(), Box<dyn Error>>
    {
        for result in self.reader()?.records() {
            let record = result?;
            trace!("{:?}", record);
        }

        Ok(())
    }

    pub fn find_single_header_index(
        &mut self,
        column_header: &String,
    ) -> Result<usize, Box<dyn Error>>
    {
        let matched_headers: Vec<usize> = self
            .reader()?
            .headers()?
            .iter()
            .enumerate()
            .filter(|(_, header)| header.eq(&column_header))
            .map(|(i, _)| i)
            .collect();

        match matched_headers.len() {
            | 0 => {
                Err(Box::new(SingleHeaderSearchError {
                    header_name: column_header.clone(),
                    error: SingleSearchError::NotFound,
                }))
            }
            | 1 => Ok(*matched_headers.first().unwrap()),
            | _ => {
                Err(Box::new(SingleHeaderSearchError {
                    header_name: column_header.clone(),
                    error: SingleSearchError::TooMany {
                        indices: matched_headers,
                    },
                }))
            }
        }
    }

    pub fn assert_equal_column_values(
        &mut self,
        column_header: &String,
    ) -> Result<(), Box<dyn Error>>
    {
        let header_i = self.find_single_header_index(&column_header)?;

        debug!("Header \"{column_header}\" found at index {header_i}");

        let mut reader = self.reader()?;
        let mut all_rows = reader.records().enumerate();

        let first = match all_rows.next() {
            | Some((_, r)) => Ok(r?),
            | None => {
                Err(Box::new(ColumnEqualityCheckError {
                    header_name: column_header.clone(),
                    error: ColumnEqualityCheckErrorOption::NoRecords,
                }))
            }
        }?;

        for (i, r) in all_rows {
            if !(r?[header_i]).eq(&first[header_i]) {
                return Err(Box::new(ColumnEqualityCheckError {
                    header_name: column_header.clone(),
                    error: ColumnEqualityCheckErrorOption::UnequalValue {
                        line_number: i + 1,
                    },
                }));
            }
        }

        Ok(())
    }
}

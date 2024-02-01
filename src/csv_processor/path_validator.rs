use log::info;
use std::{
    error::Error,
    path::PathBuf,
};

pub enum DirectoryStatus
{
    IsNotDirectory,
    IsDirectory,
}

pub enum PathValidationOptions
{
    DoesNotExist,
    Exists(DirectoryStatus),
}

pub trait PathValidator
{
    fn validate_path(
        &self,
        options: PathValidationOptions,
    ) -> Result<(), Box<dyn Error>>;
}

impl PathValidator for PathBuf
{
    fn validate_path(
        &self,
        options: PathValidationOptions,
    ) -> Result<(), Box<dyn Error>>
    {
        info!("Checking location: \"{:?}\"", self);

        match (options, self.exists()) {
            | (PathValidationOptions::Exists(_), false) => {
                Err(format!(
                    "{:?} could not be found, but is expected to exist.",
                    self.clone()
                )
                .into())
            }
            | (PathValidationOptions::DoesNotExist, true) => {
                Err(format!(
                    "{:?} should not already exist, but does.",
                    self.clone()
                )
                .into())
            }
            | (PathValidationOptions::Exists(ds), true) => {
                match (ds, self.is_dir()) {
                    | (DirectoryStatus::IsNotDirectory, false)
                    | (DirectoryStatus::IsDirectory, true) => Ok(()),
                    | (DirectoryStatus::IsDirectory, false) => {
                        return Err(format!(
                            "{:?} should be a directory, but is not.",
                            self.clone()
                        )
                        .into())
                    }
                    | (DirectoryStatus::IsNotDirectory, true) => {
                        Err(format!(
                            "{:?} should not be a directory, but it is.",
                            self.clone()
                        )
                        .into())
                    }
                }
            }
            | (PathValidationOptions::DoesNotExist, false) => Ok(()),
        }
    }
}

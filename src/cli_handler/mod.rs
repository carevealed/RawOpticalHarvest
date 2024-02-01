pub mod linux;
pub mod macos;

pub use linux::LinuxCliHandler;
use log::debug;
pub use macos::MacosCliHandler;
use std::{
    error::Error,
    path::PathBuf,
    process::Command,
};

pub trait CliHandler
{
    fn select_rom_device(&self) -> Result<String, Box<dyn Error>>;

    fn eject_tray(&self) -> Result<(), Box<dyn Error>>;

    fn get_rom_device_label(
        &self,
        dev: &String,
    ) -> Result<String, Box<dyn Error>>;

    fn dump_iso(
        &self,
        from: &PathBuf,
        to: &PathBuf,
    ) -> Result<(), Box<dyn Error>>;

    fn fix_permissions(
        &self,
        in_path: &PathBuf,
    ) -> Result<(), Box<dyn Error>>;

    // fn mount_iso(
    //     &self,
    //     iso_path: &PathBuf,
    //     mount_point: &PathBuf,
    // ) -> Result<(), Box<dyn Error>>;

    fn copy_rec(
        &self,
        from: &PathBuf,
        to: &PathBuf,
    ) -> Result<(), Box<dyn Error>>;
}

pub trait CliHandlerExtras
{
    fn run(&mut self) -> Result<String, Box<dyn Error>>;
}

impl CliHandlerExtras for Command
{
    fn run(&mut self) -> Result<String, Box<dyn Error>>
    {
        debug!("Running command: '{self:?}'");

        let result = self.output()?.stdout;
        let result = String::from_utf8(result)?;

        debug!("Command '{self:?}' produced output:");
        debug!("-----------------------------------");
        debug!("{}", result);
        debug!("-----------------------------------");

        Ok(result)
    }
}

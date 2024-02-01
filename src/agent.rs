use super::csv_processor::{
    equal_column::EqualColumnValidator,
    first_value::FirstValueFetcher,
    populated_column::PopulatedColumn,
    row_printer::RowPrinter,
};
use crate::cli_handler::*;
use log::info;
use std::{
    error::Error,
    fs,
    path::PathBuf,
};

fn get_cli_handler() -> Box<dyn CliHandler>
{
    #[cfg(target_os = "macos")]
    return Box::new(MacosCliHandler {});

    #[cfg(target_os = "linux")]
    return Box::new(LinuxCliHandler {});
}

pub struct Agent
{
    source_csv_path: PathBuf,
    dry: bool,
    cli_handler: Box<dyn CliHandler>,
}

impl Agent
{
    pub fn new(
        source_csv_path: PathBuf,
        dry: bool,
    ) -> Result<Agent, Box<dyn Error>>
    {
        let cli_handler = get_cli_handler();

        Ok(Agent {
            source_csv_path,
            dry,
            cli_handler,
        })
    }

    pub fn print_all_rows(&mut self) -> Result<(), Box<dyn Error>>
    {
        self.source_csv_path.print_all_rows()
    }

    pub fn assert_equal_column_values(
        &self,
        column_header: &String,
    ) -> Result<(), Box<dyn Error>>
    {
        self.source_csv_path
            .assert_equal_column_values(column_header)
    }

    pub fn pick_populated_column<'a>(
        &self,
        column_header_one: &'a String,
        column_header_two: &'a String,
    ) -> Result<&'a String, Box<dyn Error>>
    {
        self.source_csv_path
            .get_populated_column(column_header_one, column_header_two)
    }

    pub fn first_value(
        &mut self,
        column_header: &String,
    ) -> Result<String, Box<dyn Error>>
    {
        self.source_csv_path.get_first_value(column_header)
    }

    pub fn create_directory(
        &self,
        pdl: &PathBuf,
    ) -> Result<(), Box<dyn Error>>
    {
        info!("Create output location: {pdl:?}");
        if pdl.exists() {
            return Err(
                format!("Output location {pdl:?} already exists.",).into()
            );
        }

        if self.dry {
            info!("Dry run: Skipping creating the output directory.");
            return Ok(());
        }

        fs::create_dir(pdl)?;

        Ok(())
    }

    pub fn select_rom_device(&self) -> Result<String, Box<dyn Error>>
    {
        self.cli_handler.select_rom_device().map_err(|e| {
            format!("Error while selecting ROM device: {e}").into()
        })
    }

    pub fn eject_tray(&self) -> Result<(), Box<dyn Error>>
    {
        self.cli_handler
            .eject_tray()
            .map_err(|e| format!("Error while ejecting: {e}").into())
    }

    pub fn get_rom_device_label(
        &self,
        dev: &String,
    ) -> Result<String, Box<dyn Error>>
    {
        let label = self.cli_handler.get_rom_device_label(dev)?;

        println!("Disk has label: {label}");

        Ok(label)
    }

    pub fn dump_iso(
        &self,
        from: &PathBuf,
        to: &PathBuf,
    ) -> Result<(), Box<dyn Error>>
    {
        info!("Creating ISO from {from:?} to at {to:?}.");

        if self.dry {
            info!("Dry run: Skipping ISO dump.");
            return Ok(());
        }

        println!("Please wait...");
        let result = self.cli_handler.dump_iso(from, to).map_err(|e| {
            format!(
                "Error while dumping ISO (from: '{from:?}', to: '{to:?}'): {e}"
            )
            .into()
        });
        println!("ISO dump finished.");

        result
    }
}
